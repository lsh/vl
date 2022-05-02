use clap::Parser;
pub mod gen;
use gen::Marks;

#[derive(Parser, Debug)]
pub struct Vl {
    #[clap(subcommand)]
    pub mark: Marks,

    #[clap(short = 'w', long = "width")]
    pub width: Option<usize>,
    #[clap(short = 'h', long = "height")]
    pub height: Option<usize>,
}
