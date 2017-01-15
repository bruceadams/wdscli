extern crate clap;

use clap::Shell::{Bash, Fish, PowerShell, Zsh};
use std::fs;

include!("src/cli.rs");

fn main() {
    let out_dir = "target/completions";

    fs::create_dir(out_dir).unwrap_or(());

    for shell in [Bash, Fish, PowerShell, Zsh].iter() {
        build_cli().gen_completions("wdscli", *shell, out_dir);
    }
}
