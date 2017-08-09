
extern crate structopt;
#[macro_use]
extern crate structopt_derive;

mod cli;

use cli::WatsonDiscoveryTool;
use structopt::StructOpt;

fn main() {
    println!("Hello, world!");
    let opt = WatsonDiscoveryTool::from_args();
    println!("{:?}", opt);
}
