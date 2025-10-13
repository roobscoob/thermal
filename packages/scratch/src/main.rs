use commands::commands::reader::{Output, state::ParserState};
use facet_pretty::{FacetPretty, PrettyPrinter};
use winnow::Partial;

fn main() {
    // let data = include_bytes!("../../../examples/sharon_balling.dat");
    let data = [0x1d, 0x56, 0x00];
    let mut data = Partial::new(&data[..]);
    let mut state = ParserState::default();

    loop {
        let a = commands::commands::Command::parse(&mut data, &mut state);

        match a {
            Ok(Output::Raw(v)) => println!("Character: {:?}", v as char),
            Ok(Output::Command(c)) => {
                println!("{}", c.pretty_with(PrettyPrinter::new().with_max_depth(1)))
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}
