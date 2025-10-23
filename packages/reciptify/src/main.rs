pub mod row;
pub mod top_tracks;

use std::time::{SystemTime, UNIX_EPOCH};

use base64::{Engine, prelude::BASE64_STANDARD};
use chrono::{Datelike, Duration};
use clap::Parser;
use facet_pretty::FacetPretty;
use futures_core::Stream;
use futures_util::StreamExt;
use serde::Deserialize;
use thermal::{
    commands::{Command, reader::Output},
    emulator::Emulator,
    state::{
        delta::Delta,
        effect::{feed::Feed, print::Write},
    },
    types::{
        cut_mode::{CutMode, CuttingShape},
        font::Font,
        justification::Justification,
    },
};
use thermal_emulator_tm_t88v::device::TmT88v;

use crate::{row::Row, top_tracks::TopTracks};

#[derive(clap::Parser, Debug)]
struct Args {
    username: String,
}

#[derive(Deserialize, Debug)]
struct Response {
    toptracks: TopTracks,
}

pub async fn send(data: &[Output]) {
    let data = data
        .iter()
        .map(|v| match v {
            Output::Raw(x) => vec![*x],
            Output::Command(c) => {
                let mut x = vec![];
                c.write_to(&mut x).unwrap();
                x
            }
        })
        .flatten()
        .collect::<Vec<u8>>();

    reqwest::Client::new()
        .post("http://10.100.1.159:8000/escpos")
        .body(format!(
            r#"{{ "buffer": "{}" }}"#,
            BASE64_STANDARD.encode(data)
        ))
        .send()
        .await
        .unwrap();
}

const CHAR_WIDTH: usize = 42;

fn format_duration(d: Duration) -> String {
    let total_seconds = d.num_seconds();
    let sign = if total_seconds < 0 { "-" } else { "" };
    let total_seconds = total_seconds.abs();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{sign}{hours}:{minutes:02}:{seconds:02}")
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let client = reqwest::Client::new();

    let res: Response = client.get("https://ws.audioscrobbler.com/2.0/?method=user.gettoptracks&user=roobscoob&period=6month&limit=25&api_key=c85bdae42679d038bb1ff515aaf90ce7&format=json").send().await.unwrap().json().await.unwrap();

    println!("{res:#?}");

    let mut device = TmT88v::default();

    let now = chrono::Local::now();

    let item_count: u32 = res
        .toptracks
        .track
        .iter()
        .map(|v| v.playcount.parse::<u32>().unwrap())
        .sum();

    let total: u32 = res
        .toptracks
        .track
        .iter()
        .map(|v| v.duration.parse::<u32>().unwrap() * v.playcount.parse::<u32>().unwrap())
        .sum();

    let mut commands = device
        .apply(
            Delta::empty()
                .with(
                    Write::from_str("RECEIPTIFY")
                        .with_justification(Justification::Centered)
                        .with_scale(3, 3)
                        .with_font(Font::B),
                )
                .with(Feed::lines(2))
                .with(
                    Write::from_str("LAST 6 MONTHS")
                        .with_justification(Justification::Centered)
                        .with_scale(2, 2)
                        .with_font(Font::B),
                )
                .with(Feed::lines(2))
                .with(Write::from_str(&format!(
                    "ORDER #0001 FOR {}",
                    args.username.to_uppercase()
                )))
                .with(Feed::lines(1))
                .with(Write::from_str(&format!(
                    "{}, {} {}, {}",
                    now.weekday().to_string().to_uppercase(),
                    format!("{:?}", chrono::Month::try_from(now.month() as u8).unwrap())
                        .to_uppercase(),
                    now.day(),
                    now.year()
                )))
                .with(Feed::lines(1))
                .with(Write::from_str(&"-".repeat(CHAR_WIDTH)))
                .with(Feed::lines(1))
                .with(
                    Row::new()
                        .with_cell(4, "QTY".to_string(), pad::Alignment::Left)
                        .with_cell(32, "ITEM".to_string(), pad::Alignment::Left)
                        .with_gap()
                        .with_cell(5, "AMT".to_string(), pad::Alignment::Right),
                )
                .with(Feed::lines(1))
                .with(Write::from_str(&"-".repeat(CHAR_WIDTH)))
                .with(res.toptracks)
                .with(Write::from_str(&"-".repeat(CHAR_WIDTH)))
                .with(
                    Row::new()
                        .with_cell(21, "ITEM COUNT:".to_string(), pad::Alignment::Left)
                        .with_cell(21, item_count.to_string(), pad::Alignment::Right),
                )
                .with(
                    Row::new()
                        .with_cell(21, "TOTAL:".to_string(), pad::Alignment::Left)
                        .with_cell(
                            21,
                            format_duration(Duration::seconds(total as i64)),
                            pad::Alignment::Right,
                        ),
                )
                .with(Write::from_str("CARD #: **** **** **** 2023"))
                .with(Feed::lines(1))
                .with(Write::from_str("AUTH CODE: 123421"))
                .with(Feed::lines(1))
                .with(Write::from_str(&format!(
                    "CARDHOLDER: {}",
                    args.username.to_uppercase()
                ))),
        )
        .unwrap();

    println!("{}", commands.pretty());

    commands.extend_from_slice(&[
        Output::Command(Command::CarriageReturn),
        Output::Command(Command::LineFeed),
        Output::Command(Command::SelectCutModeAndCutPaper(CutMode::FeedAndCut(
            10,
            CuttingShape::Full,
        ))),
    ]);

    send(&commands).await;
}
