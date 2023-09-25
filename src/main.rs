#![allow(unused)]
use std::process;

use clap::{CommandFactory, Parser};
use cli::{Cli, Runnable};

mod cli;

fn main() {
    match Cli::try_parse() {
        Ok(cli) => {
            cli.run();
        }
        Err(e) => {
            e.print().expect("Error writing Error");
            process::exit(0)
        }
    }
}
