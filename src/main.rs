use std::process;
use clap::Parser;
use text_colorizer::Colorize;
use qlox::{Args, Lox};

fn main() {
    if let Err(e) = Lox::new(Args::parse()).start() {
        eprintln!("{}: {e}", "error".red().bold());
        process::exit(e.exit_code());
    }
}
