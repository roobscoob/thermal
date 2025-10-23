use pad::{Alignment, PadStr};
use thermal::state::effect::{IntoEffects, print::Write};
use unicode_segmentation::UnicodeSegmentation;

pub struct Row {
    contents: Vec<(u8, String, Alignment)>,
}

impl Row {
    pub fn new() -> Row {
        Self { contents: vec![] }
    }

    pub fn with_cell(mut self, width: u8, contents: String, alignment: Alignment) -> Self {
        self.contents.push((width, contents, alignment));
        self
    }

    pub fn with_gap(mut self) -> Self {
        self.contents.push((1, "".to_string(), Alignment::Left));
        self
    }

    fn next_line(&mut self) -> Option<String> {
        let mut current_line = String::new();
        let mut nonempty = false;

        for (width, content, alignment) in self.contents.iter_mut() {
            let col = *width as usize;

            if !content.is_empty() {
                nonempty = true;
            }

            // If it fits in bytes, pad to exact byte width and consume.
            if content.as_bytes().len() <= col {
                let padded = content.pad_to_width_with_alignment(col, *alignment);
                current_line.push_str(&padded);
                content.clear();
                continue;
            }

            // Otherwise, take up to `col` BYTES, preferring word boundaries.
            let left = take_prefix_by_words_bytes(content, col);
            current_line.push_str(&left.pad_to_width_with_alignment(col, *alignment));
        }

        // NOTE: uppercasing after padding can change byte length for some chars (e.g., ß → SS).
        nonempty.then(|| current_line.to_uppercase())
    }
}

impl IntoEffects for Row {
    fn as_effects(mut self) -> impl Iterator<Item = thermal::state::effect::Effect> {
        std::iter::from_fn(move || self.next_line()).flat_map(|v| Write::from_str(&v).as_effects())
    }
}

/// Take up to `max_bytes` from the front of `s`, preferring Unicode word boundaries.
/// Never splits inside a UTF-8 code point. Mutates `s` to the remainder.
/// Also trims leading whitespace from the remainder so next line doesn't start with spaces.
fn take_prefix_by_words_bytes(s: &mut String, max_bytes: usize) -> String {
    let bytes = s.as_str();
    let mut used = 0usize;
    let mut end = 0usize; // byte index into `bytes`

    // 1) Prefer cutting on word boundaries (words, spaces, punctuation as pieces).
    for (idx, piece) in UnicodeSegmentation::split_word_bound_indices(bytes) {
        let plen = piece.as_bytes().len();
        if used + plen > max_bytes {
            if used == 0 {
                // 2) First piece already too big: cut at the last valid char boundary ≤ max_bytes.
                let mut cut = max_bytes.min(bytes.len());
                while cut > 0 && !bytes.is_char_boundary(cut) {
                    cut -= 1;
                }
                end = cut;
            }
            break;
        }
        used += plen;
        end = idx + plen;
        if used == max_bytes {
            break;
        }
    }

    // If loop took nothing but the string is shorter than max_bytes, take all.
    if end == 0 && bytes.len() <= max_bytes {
        end = bytes.len();
    }

    let left = bytes[..end].to_string();
    let mut right = bytes[end..].to_string();

    // Trim leading whitespace so next line doesn't start with spaces/tabs.
    if !right.is_empty() {
        right = right.trim_start().to_string();
    }

    *s = right;
    left
}
