use std::{fmt::Alignment, time::Duration};

use base64::{Engine, prelude::BASE64_STANDARD};
use facet::{Facet, UserType};
use facet_pretty::{FacetPretty, PrettyPrinter};
use nusb::transfer::{Bulk, Direction, In, Out};
use reqwest::blocking::Client;
use strum::EnumMessage;
use thermal::{
    commands::{
        Command,
        reader::{Output, state::ParserState},
    },
    emulator::Emulator,
    state::{delta::Delta, effect::print::Write},
    types::{
        character_set::{AsciiVariant, Codepage},
        cut_mode::{CutMode, CuttingShape},
        font::Font,
        justification::Justification,
        requested_status::RequestedStatus,
    },
};
use thermal_emulator_tm_t88v::device::TmT88v;
use thermal_encoding::{encode_str, encoding::PartialUnicodeEncoding};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use winnow::Partial;

pub fn send(data: &[Output]) {
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

    Client::new()
        .post("http://10.100.1.159:8000/escpos")
        .body(format!(
            r#"{{ "buffer": "{}" }}"#,
            BASE64_STANDARD.encode(data)
        ))
        .send()
        .unwrap();
}

// #[tokio::main]
// async fn main() {
//     let device = nusb::list_devices()
//         .await
//         .unwrap()
//         .find(|v| v.product_string() == Some("TM-T88V"))
//         .unwrap()
//         .open()
//         .await
//         .unwrap();

//     let interface = device
//         .active_configuration()
//         .unwrap()
//         .interfaces()
//         .next()
//         .unwrap()
//         .interface_number();

//     let interface = device.claim_interface(interface).await.unwrap();

//     let endpoints = interface
//         .descriptor()
//         .unwrap()
//         .endpoints()
//         .collect::<Vec<_>>();

//     let output = endpoints
//         .iter()
//         .find(|v| v.direction() == Direction::Out)
//         .unwrap();

//     let input = endpoints
//         .iter()
//         .find(|v| v.direction() == Direction::In)
//         .unwrap();

//     let mut output = interface
//         .endpoint::<Bulk, Out>(output.address())
//         .unwrap()
//         .writer(output.max_packet_size());

//     let mut input = interface
//         .endpoint::<Bulk, In>(input.address())
//         .unwrap()
//         .reader(input.max_packet_size());

// }

// #[tokio::main]
// async fn main() {
//     let device = nusb::list_devices()
//         .await
//         .unwrap()
//         .find(|v| v.product_string() == Some("TM-T88V"))
//         .unwrap()
//         .open()
//         .await
//         .unwrap();

//     let interface = device
//         .active_configuration()
//         .unwrap()
//         .interfaces()
//         .next()
//         .unwrap()
//         .interface_number();

//     let interface = device.claim_interface(interface).await.unwrap();

//     let endpoints = interface
//         .descriptor()
//         .unwrap()
//         .endpoints()
//         .collect::<Vec<_>>();

//     let output = endpoints
//         .iter()
//         .find(|v| v.direction() == Direction::Out)
//         .unwrap();

//     let input = endpoints
//         .iter()
//         .find(|v| v.direction() == Direction::In)
//         .unwrap();

//     let mut output = interface
//         .endpoint::<Bulk, Out>(output.address())
//         .unwrap()
//         .writer(output.max_packet_size());

//     let mut input = interface
//         .endpoint::<Bulk, In>(input.address())
//         .unwrap()
//         .reader(input.max_packet_size());

//     let mut c = vec![];

//     let x1 = [
//         Output::Command(Command::InitializePrinter),
//         Output::Command(Command::RequestStatus(RequestedStatus::RollPaperSensor)),
//     ];

//     // let x2 = [
//     //     Output::Command(Command::CarriageReturn),
//     //     Output::Command(Command::LineFeed),
//     //     Output::Command(Command::SelectCutModeAndCutPaper(CutMode::FeedAndCut(
//     //         10,
//     //         CuttingShape::Full,
//     //     ))),
//     // ];

//     for item in x1 {
//         match item {
//             Output::Raw(v) => c.push(v),
//             Output::Command(cx) => {
//                 cx.write_to(&mut c).unwrap();
//             }
//         }
//     }

//     c.extend_from_slice(&[0x10, 0x04, 0x04]);

//     println!("{}", c.pretty());

//     output.write(&c).await.unwrap();

//     output.flush().await.unwrap();

//     loop {
//         let mut x = [0];

//         input.read(&mut x).await.unwrap();

//         println!("Recv: {}", x[0]);
//     }
// }

pub fn main() {
    let mut e = TmT88v::default();

    let mut x = e
        .apply(
            Delta::empty().with(
                Write::from_str(&"Hello, セカイ!")
                    .with_font(Font::B)
                    .with_justification(Justification::LeftJustified)
                    .with_scale(3, 3),
            ),
        )
        .unwrap();

    x.extend_from_slice(&[
        Output::Command(Command::CarriageReturn),
        Output::Command(Command::LineFeed),
        Output::Command(Command::SelectCutModeAndCutPaper(CutMode::FeedAndCut(
            10,
            CuttingShape::Full,
        ))),
    ]);

    println!("{}", x.pretty());

    send(&x);
}
