use std::ops::Not;

use chrono::Duration;
use clap::builder::Str;
use pad::PadStr;
use serde::Deserialize;
use thermal::state::effect::IntoEffects;

use crate::row::Row;

#[derive(Deserialize, Debug)]
pub struct Artist {
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub playcount: String,
    pub name: String,
    pub duration: String,
    pub artist: Artist,
}

#[derive(Deserialize, Debug)]
pub struct TopTracks {
    pub track: Vec<Track>,
}

fn format_duration(d: Duration) -> String {
    let total_seconds = d.num_seconds();
    let sign = if total_seconds < 0 { "-" } else { "" };
    let total_seconds = total_seconds.abs();

    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;

    format!("{sign}{minutes:02}:{seconds:02}")
}

impl IntoEffects for TopTracks {
    fn as_effects(self) -> impl Iterator<Item = thermal::state::effect::Effect> {
        self.track.into_iter().flat_map(|v| {
            let duration = Duration::seconds(v.duration.parse().unwrap());

            Row::new()
                .with_cell(4, v.playcount, pad::Alignment::Left)
                .with_cell(
                    32,
                    format!("{} - {}", v.name, v.artist.name),
                    pad::Alignment::Left,
                )
                .with_gap()
                .with_cell(
                    5,
                    duration
                        .is_zero()
                        .not()
                        .then(|| format_duration(duration))
                        .unwrap_or_default(),
                    pad::Alignment::Right,
                )
                .as_effects()
        })
    }
}
