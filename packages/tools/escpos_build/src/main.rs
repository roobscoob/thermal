use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use reqwest::blocking::Client;
use scraper::ElementRef;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs,
    path::{Path, PathBuf},
};
use url::Url;

const INDEX_URL: &str =
    "https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/commands.html";
const BASE_URL: &str = "https://download4.epson.biz/sec_pubs/pos/reference_en/escpos/";

#[derive(Parser, Debug)]
#[command(
    name = "escpos_build",
    version,
    about = "Fetch + build ESC/POS command spec"
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Download index + all command pages into a folder
    Fetch {
        /// Output folder (will be created)
        out_dir: PathBuf,
    },
    /// Parse a local folder (from `fetch`) and build spec JSON
    Build {
        /// Folder that contains the downloaded HTML files
        src_dir: PathBuf,
        /// Output JSON path (default: ./spec/commands.json)
        #[arg(short = 'o', long = "out")]
        out: Option<PathBuf>,

        /// One or more JSON override files (later files win)
        #[arg(short = 'L', long = "labels")]
        labels: Vec<PathBuf>,
    },
}

#[derive(Debug, Serialize)]
struct Root {
    categories: Vec<String>,
    commands: Vec<Command>,
}

#[derive(Debug, Serialize)]
struct Command {
    id: String,
    name: String,
    category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    to_string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detailed_message: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    notes: Vec<String>,
    #[serde(default)]
    obsolete: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
struct IndexEntry {
    id: String,
    filename: String,
    category: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
struct CommandOverride {
    to_string: Option<String>,
    message: Option<String>,
    name: Option<String>,
    category: Option<String>,
    detailed_message: Option<String>,
    notes: Option<Vec<String>>,
    obsolete: Option<bool>,
    content: Option<Vec<String>>,
}

#[derive(Debug, Default)]
struct LoadedOverrides {
    per_id: HashMap<String, CommandOverride>,
    categories: Option<Vec<String>>, // from "_categories"
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Fetch { out_dir } => cmd_fetch(&out_dir),
        Cmd::Build {
            src_dir,
            out,
            labels,
        } => cmd_build(&src_dir, out, labels),
    }
}

fn load_overrides(paths: &[PathBuf]) -> Result<LoadedOverrides> {
    let mut out = LoadedOverrides::default();

    for p in paths {
        let txt = fs::read_to_string(p)
            .with_context(|| format!("read overrides file {}", p.display()))?;
        let val: serde_json::Value =
            serde_json::from_str(&txt).with_context(|| format!("parse json {}", p.display()))?;

        let obj = val
            .as_object()
            .ok_or_else(|| anyhow!("override file {} must be a JSON object", p.display()))?;

        for (k, v) in obj {
            if k == "_categories" {
                let arr = v
                    .as_array()
                    .ok_or_else(|| anyhow!("_categories must be an array of strings"))?;
                let cats: Vec<String> = arr
                    .iter()
                    .map(|x| x.as_str().unwrap_or_default().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                out.categories = Some(cats);
                continue;
            }

            // normal per-id override
            let ov: CommandOverride = serde_json::from_value(v.clone())
                .with_context(|| format!("invalid override for id '{}'", k))?;
            // merge (later files win)
            out.per_id
                .entry(k.clone())
                .and_modify(|e| merge_override_into(e, &ov))
                .or_insert(ov);
        }
    }

    Ok(out)
}

fn merge_override_into(base: &mut CommandOverride, add: &CommandOverride) {
    if add.to_string.is_some() {
        base.to_string = add.to_string.clone();
    }
    if add.message.is_some() {
        base.message = add.message.clone();
    }
    if add.name.is_some() {
        base.name = add.name.clone();
    }
    if add.category.is_some() {
        base.category = add.category.clone();
    }
    if add.detailed_message.is_some() {
        base.detailed_message = add.detailed_message.clone();
    }
    if add.notes.is_some() {
        base.notes = add.notes.clone();
    }
    if add.obsolete.is_some() {
        base.obsolete = add.obsolete;
    }
    if add.content.is_some() {
        base.content = add.content.clone();
    }
}

fn cmd_fetch(out_dir: &Path) -> Result<()> {
    fs::create_dir_all(out_dir).with_context(|| format!("create {}", out_dir.display()))?;
    let pages_dir = out_dir.join("pages");
    fs::create_dir_all(&pages_dir)?;

    let client = Client::builder()
        .user_agent("escpos_build/0.2 (rust)")
        .build()?;

    eprintln!("fetching index: {INDEX_URL}");
    let index_html = client.get(INDEX_URL).send()?.error_for_status()?.text()?;
    fs::write(out_dir.join("commands.html"), &index_html)?;

    // Parse links from the index so we know which pages to fetch.
    let index_doc = Html::parse_document(&index_html);
    let entries = parse_index_links(&index_doc)?;

    for (i, ent) in entries.iter().enumerate() {
        let url = Url::parse(BASE_URL)?.join(&ent.filename)?;
        eprintln!("[{}/{}] {}", i + 1, entries.len(), url);
        let html = client.get(url).send()?.error_for_status()?.text()?;
        fs::write(pages_dir.join(&ent.filename), html)?;
    }

    eprintln!(
        "done. saved index to {}/commands.html and pages to {}/pages/",
        out_dir.display(),
        out_dir.display()
    );
    Ok(())
}

fn cmd_build(src_dir: &Path, out: Option<PathBuf>, labels: Vec<PathBuf>) -> Result<()> {
    let overrides = load_overrides(&labels)?;
    let index_path = src_dir.join("commands.html");
    let pages_dir = src_dir.join("pages");

    if !index_path.exists() {
        return Err(anyhow!("index not found at {}", index_path.display()));
    }
    if !pages_dir.is_dir() {
        return Err(anyhow!("pages folder not found at {}", pages_dir.display()));
    }

    let index_html = fs::read_to_string(&index_path)?;
    let index_doc = Html::parse_document(&index_html);
    let entries = parse_index_links(&index_doc)?;

    // For category ordering, we use first-seen order.
    let mut categories: Vec<String> = Vec::new();
    for e in &entries {
        if !categories.contains(&e.category) {
            categories.push(e.category.clone());
        }
    }

    // Build commands by parsing each local page
    let mut commands = Vec::with_capacity(entries.len());
    for (i, ent) in entries.iter().enumerate() {
        let path = pages_dir.join(&ent.filename);
        eprintln!(
            "[{}/{}] building from {}",
            i + 1,
            entries.len(),
            path.display()
        );
        let html = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let (name, detailed, notes, obsolete) = parse_command_page_minimal(&html)?;
        commands.push(Command {
            id: ent.id.clone(), // <-- NEW
            name,
            category: ent.category.clone(),
            to_string: None,
            message: None,
            detailed_message: detailed,
            notes,
            obsolete,
            content: None,
        });
    }

    // If overrides provided categories, use them; else keep first-seen order
    if let Some(ov_cats) = &overrides.categories {
        // Ensure uniqueness and preserve provided ordering
        let mut seen = std::collections::BTreeSet::new();
        let mut new_cats = Vec::new();
        for c in ov_cats {
            if seen.insert(c.clone()) {
                new_cats.push(c.clone());
            }
        }
        categories = new_cats;
    }

    // Apply per-id overrides and track changes
    let mut applied = Vec::new();
    for cmd in &mut commands {
        if let Some(ov) = overrides.per_id.get(&cmd.id) {
            let mut changed = false;

            if let Some(v) = &ov.name {
                if &cmd.name != v {
                    cmd.name = v.clone();
                    changed = true;
                }
            }
            if let Some(v) = &ov.to_string {
                if cmd.to_string.as_ref() != Some(v) {
                    cmd.to_string = Some(v.clone());
                    changed = true;
                }
            }
            if let Some(v) = &ov.message {
                if cmd.message.as_ref() != Some(v) {
                    cmd.message = Some(v.clone());
                    changed = true;
                }
            }
            if let Some(v) = &ov.detailed_message {
                if cmd.detailed_message.as_ref() != Some(v) {
                    cmd.detailed_message = Some(v.clone());
                    changed = true;
                }
            }
            if let Some(v) = &ov.notes {
                if &cmd.notes != v {
                    cmd.notes = v.clone();
                    changed = true;
                }
            }
            if let Some(v) = ov.obsolete {
                if cmd.obsolete != v {
                    cmd.obsolete = v;
                    changed = true;
                }
            }
            if let Some(v) = &ov.content {
                if cmd.content.as_ref() != Some(v) {
                    cmd.content = Some(v.clone());
                    changed = true;
                }
            }

            if let Some(new_cat) = &ov.category {
                if &cmd.category != new_cat {
                    cmd.category = new_cat.clone();
                    changed = true;
                    // ensure category exists
                    if !categories.contains(new_cat) {
                        categories.push(new_cat.clone());
                    }
                }
            }

            if changed {
                applied.push(cmd.id.clone());
            }
        }
    }

    // Tiny report
    if !applied.is_empty() {
        eprintln!("Applied overrides for {} ids:", applied.len());
        for id in applied {
            eprintln!("  - {}", id);
        }
    }

    let root = Root {
        categories,
        commands,
    };
    let json = serde_json::to_string_pretty(&root)?;

    let out_path = out.unwrap_or_else(|| PathBuf::from("spec").join("commands.json"));
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&out_path, json)?;
    eprintln!("wrote {}", out_path.display());
    Ok(())
}

// -------- parsing helpers (index) --------

fn parse_index_links(doc: &Html) -> Result<Vec<IndexEntry>> {
    // The index page is a table of commands. Each row has a link to `<something>.html` and a Category cell.
    let a_sel = Selector::parse("a[href$='.html']").unwrap();
    let td_sel = Selector::parse("td").unwrap();

    // collect unique (filename, category)
    let mut out = Vec::<IndexEntry>::new();

    for a in doc.select(&a_sel) {
        let href = a.value().attr("href").unwrap_or_default().trim();
        // skip folder links; we only want same-folder pages like "ht.html"
        if href.is_empty() || href.contains('/') {
            continue;
        }

        let id = href.trim_end_matches(".html").to_string();

        // Climb to the row
        let tr = match find_ancestor_tag(&a, "tr") {
            Some(er) => er,
            None => continue,
        };

        // Gather cells’ text so we can find the category column.
        let mut cells: Vec<String> = Vec::new();
        let mut link_cell_idx: Option<usize> = None;
        for (i, td) in tr.select(&td_sel).enumerate() {
            let has_link = td.select(&a_sel).next().is_some();
            if has_link {
                link_cell_idx = Some(i);
            }
            let text = normalize_ws(&td.text().collect::<Vec<_>>().join(" "));
            cells.push(text);
        }

        // Heuristic: category is the last non-empty cell that is not the link cell.
        let mut category = String::new();
        if let Some(li) = link_cell_idx {
            for (i, cell) in cells.iter().enumerate().rev() {
                if i != li && !cell.is_empty() {
                    category = cell.clone();
                    break;
                }
            }
        }
        if category.is_empty() {
            category = "Uncategorized".to_string();
        }

        // De-dup on filename (some appear multiple times)
        if !out.iter().any(|e| e.filename == href) {
            out.push(IndexEntry {
                id,
                filename: href.to_string(),
                category,
            });
        }
    }
    Ok(out)
}

fn find_ancestor_tag<'a>(
    start: &scraper::ElementRef<'a>,
    tag: &str,
) -> Option<scraper::ElementRef<'a>> {
    let mut cur = Some(start.clone());
    while let Some(er) = cur {
        if er.value().name().eq_ignore_ascii_case(tag) {
            return Some(er);
        }
        cur = er
            .ancestors()
            .skip(1)
            .find_map(|n| scraper::ElementRef::wrap(n));
    }
    None
}

// -------- parsing helpers (detail page) --------

fn parse_command_page_minimal(html: &str) -> Result<(String, Option<String>, Vec<String>, bool)> {
    let doc = Html::parse_document(html);
    let h_sel = Selector::parse("h2, h3").unwrap();

    let name =
        extract_block_after(&doc, &h_sel, "[Name]")?.unwrap_or_else(|| "Unknown".to_string());
    let detailed = extract_block_after(&doc, &h_sel, "[Description]")?;
    let notes = extract_notes(&doc, &h_sel)?;
    let obsolete = detect_obsolete(&doc);

    Ok((name, detailed, notes, obsolete))
}

// Finds the closest ancestor whose class list contains `class_name`.
fn find_ancestor_with_class<'a>(
    start: &ElementRef<'a>,
    class_name: &str,
) -> Option<ElementRef<'a>> {
    for anc in start.ancestors().filter_map(ElementRef::wrap) {
        if element_has_class(&anc, class_name) {
            return Some(anc);
        }
    }
    None
}

fn element_has_class(el: &ElementRef<'_>, cls: &str) -> bool {
    el.value()
        .attr("class")
        .map(|c| c.split_whitespace().any(|x| x.eq_ignore_ascii_case(cls)))
        .unwrap_or(false)
}

fn extract_notes(doc: &Html, h_sel: &Selector) -> Result<Vec<String>> {
    let ul_li_sel = Selector::parse("ul li").unwrap();

    // Find the [Notes] heading
    for h in doc.select(h_sel) {
        let t = h.text().collect::<Vec<_>>().join(" ");
        if !t.contains("[Notes]") {
            continue;
        }

        // Preferred: search within the enclosing <div class="Header2"> ... </div>
        if let Some(section) = find_ancestor_with_class(&h, "Header2") {
            let mut out = Vec::new();
            for li in section.select(&ul_li_sel) {
                let txt = normalize_ws(&li.text().collect::<Vec<_>>().join(" "));
                if !txt.is_empty() {
                    out.push(txt);
                }
            }
            if !out.is_empty() {
                return Ok(out);
            }
            // If Header2 exists but no ULs, fall through to sibling scan.
        }

        // Fallback: walk forward siblings until next h2/h3; collect the first UL’s LIs
        let ul_sel = Selector::parse("ul").unwrap();
        let li_sel = Selector::parse("li").unwrap();
        let mut sib = h.next_sibling();
        while let Some(n) = sib {
            if let Some(el) = n.value().as_element() {
                let tag = el.name().to_ascii_lowercase();
                if tag == "h2" || tag == "h3" {
                    break;
                }
                if let Some(er) = ElementRef::wrap(n) {
                    if let Some(ul) = er.select(&ul_sel).next() {
                        let mut out = Vec::new();
                        for li in ul.select(&li_sel) {
                            let txt = normalize_ws(&li.text().collect::<Vec<_>>().join(" "));
                            if !txt.is_empty() {
                                out.push(txt);
                            }
                        }
                        if !out.is_empty() {
                            return Ok(out);
                        }
                    }
                }
            }
            sib = n.next_sibling();
        }
    }

    Ok(Vec::new())
}

fn extract_block_after(doc: &Html, h_sel: &Selector, marker: &str) -> Result<Option<String>> {
    for h in doc.select(h_sel) {
        let t = h.text().collect::<Vec<_>>().join(" ");
        if t.contains(marker) {
            // Next siblings until we hit non-heading text
            let mut sib = h.next_sibling();
            while let Some(n) = sib {
                if let Some(el) = n.value().as_element() {
                    let tag = el.name();
                    if tag.eq_ignore_ascii_case("h2") || tag.eq_ignore_ascii_case("h3") {
                        break;
                    }
                    if let Some(er) = scraper::ElementRef::wrap(n) {
                        let txt = normalize_ws(&er.text().collect::<Vec<_>>().join(" "));
                        if !txt.is_empty() {
                            return Ok(Some(txt));
                        }
                    }
                }
                sib = n.next_sibling();
            }
        }
    }
    Ok(None)
}

fn detect_obsolete(doc: &Html) -> bool {
    // Very weak heuristic: if the page has a heading containing "Obsolete" or "Deprecated"
    // or if a banner exists. Easy to tighten later if needed.
    let h_sel = Selector::parse("h1, h2, h3, h4").unwrap();
    for h in doc.select(&h_sel) {
        let t = h.text().collect::<Vec<_>>().join(" ").to_lowercase();
        if t.contains("obsolete") || t.contains("deprecated") {
            return true;
        }
    }
    false
}

// -------- misc --------

fn normalize_ws(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_space = false;
    for ch in s.chars() {
        if ch.is_whitespace() {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            out.push(ch);
            prev_space = false;
        }
    }
    out.trim().to_string()
}
