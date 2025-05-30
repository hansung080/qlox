use std::process;
use clap::Parser;
use qlox::{Args, Lox};

fn main() {
    if let Err(e) = Lox::new(Args::parse()).start() {
        eprint!("{e}");
        process::exit(e.exit_code());
    }
}
