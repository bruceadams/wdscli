#![feature(attr_literals)]

extern crate clap;
extern crate stomp;
#[macro_use]
extern crate stomp_macros;

mod cli;

use cli::{Verb, WatsonDiscoveryTool};
use stomp::ParseApp;

fn main() {
    let opt = WatsonDiscoveryTool::parse();
    println!("{:?}", opt);
    match opt.verb {
        Verb::AddDocument(_) => println!("yo"),
        Verb::CrawlerConfiguration(_) => println!("yo"),
        Verb::CreateCollection(_) => println!("yo"),
        Verb::CreateConfiguration(_) => println!("yo"),
        Verb::CreateEnvironment(_) => println!("yo"),
        Verb::DeleteCollection(_) => println!("yo"),
        Verb::DeleteConfiguration(_) => println!("yo"),
        Verb::DeleteDocument(_) => println!("yo"),
        Verb::DeleteEnvironment(_) => println!("yo"),
        Verb::GenerateCompletions(_) => println!("yo"),
        Verb::Notices(_) => println!("yo"),
        Verb::Overview(_) => println!("yo"),
        Verb::Preview(_) => println!("yo"),
        Verb::Query(_) => println!("yo"),
        Verb::ShowCollection(_) => println!("yo"),
        Verb::ShowConfiguration(_) => println!("yo"),
        Verb::ShowDocument(_) => println!("yo"),
        Verb::ShowEnvironment(_) => println!("yo"),
    }
}
