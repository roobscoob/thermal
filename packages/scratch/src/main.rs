use std::any::Any;

use base64::{Engine, prelude::BASE64_STANDARD};
use facet::{Facet, UserType};
use facet_pretty::{FacetPretty, PrettyPrinter};
use nusb::transfer::{Bulk, Direction, In, Out};
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
    },
};
use thermal_emulator_tm_t88v::device::TmT88v;
use thermal_encoding::{encode_str, encoding::PartialUnicodeEncoding};
use tokio::io::AsyncWriteExt;
use winnow::Partial;

#[tokio::main]
async fn main() {
    let device = nusb::list_devices()
        .await
        .unwrap()
        .find(|v| v.product_string() == Some("TM-T88V"))
        .unwrap()
        .open()
        .await
        .unwrap();

    let interface = device
        .active_configuration()
        .unwrap()
        .interfaces()
        .next()
        .unwrap()
        .interface_number();

    let interface = device.claim_interface(interface).await.unwrap();

    let endpoints = interface
        .descriptor()
        .unwrap()
        .endpoints()
        .collect::<Vec<_>>();

    let output = endpoints
        .iter()
        .find(|v| v.direction() == Direction::Out)
        .unwrap();

    let input = endpoints
        .iter()
        .find(|v| v.direction() == Direction::In)
        .unwrap();

    let mut output = interface
        .endpoint::<Bulk, Out>(output.address())
        .unwrap()
        .writer(output.max_packet_size());

    let input = interface
        .endpoint::<Bulk, In>(input.address())
        .unwrap()
        .reader(input.max_packet_size());

    // real shit

    let mut emulator = TmT88v::default();

    let result = emulator
        .apply(Delta::empty().with(Write::from_str("Hello, セカイ! C'est génial")))
        .unwrap();

    Command::InitializePrinter.write_to(&mut output).unwrap();

    println!("{}", result.pretty());

    for item in result {
        match item {
            Output::Raw(v) => output.write(&[v]).await.unwrap(),
            Output::Command(c) => c.write_to(&mut output).unwrap(),
        };
    }

    Command::CarriageReturn.write_to(&mut output).unwrap();
    Command::LineFeed.write_to(&mut output).unwrap();
    Command::SelectCutModeAndCutPaper(CutMode::FeedAndCut(10, CuttingShape::Full))
        .write_to(&mut output)
        .unwrap();

    output.flush().await.unwrap();
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

//     let input = interface
//         .endpoint::<Bulk, In>(input.address())
//         .unwrap()
//         .reader(input.max_packet_size());

//     let mut c = vec![];

//     let x1 = [Output::Command(Command::InitializePrinter)];

//     let x2 = [
//         Output::Command(Command::CarriageReturn),
//         Output::Command(Command::LineFeed),
//         Output::Command(Command::SelectCutModeAndCutPaper(CutMode::FeedAndCut(
//             10,
//             CuttingShape::Full,
//         ))),
//     ];

//     println!("{}", c.pretty());

//     output.write(&c).await.unwrap();

//     output.flush().await.unwrap();
// }
