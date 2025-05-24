mod consts;
mod types;
mod loc;
mod token;
mod scanner;

use std::{fs, io, result};
use std::io::Write;
use clap::Parser;
use text_colorizer::Colorize;
use thiserror::Error;
use crate::consts::exitcode;
use crate::scanner::Scanner;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
}

impl Error {
    pub fn exit_code(&self) -> i32 {
        use Error::*;
        match self {
            Io(_) => exitcode::IO_ERR,
        }
    }
}

// These `Display` and `From` traits are implemented by the `error` and `from` attributes of the `thiserror` crate.
// use std::fmt::{self, Display, Formatter};
//
// impl Display for Error {
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         use Error::*;
//         match self {
//             Io(e) => write!(f, "{e}"),
//         }
//     }
// }
//
// impl From<io::Error> for Error {
//     fn from(value: io::Error) -> Self {
//         Error::Io(value)
//     }
// }

/// A tree-walk interpreter for the Lox programming language
#[derive(Parser, Debug)]
#[command(name = Lox::name(), version = Lox::version(), author, about)]
pub struct Args {
    /// A Lox file to run
    #[arg(value_name = "FILE")]
    pub filename: Option<String>,
}

pub struct Lox {
    args: Args,
}

impl Lox {
    pub fn new(args: Args) -> Self {
        Lox { args }
    }

    pub fn name() -> &'static str {
        option_env!("CARGO_PKG_NAME").unwrap_or("qlox")
    }

    pub fn version() -> &'static str {
        option_env!("CARGO_PKG_VERSION").unwrap_or("undefined")
    }

    pub fn start(&self) -> Result<()> {
        match self.args.filename {
            Some(ref filename) => self.run_file(filename),
            None => self.run_prompt(),
        }
    }

    fn run_file(&self, path: &str) -> Result<()> {
        self.run(fs::read(path)?)
    }

    fn run_prompt(&self) -> Result<()> {
        println!("Welcome to `{} {}` REPL.", Lox::name().blue(), Lox::version().blue());
        println!("Type `{}`, `{}`, or `{}` in order to issue a command.",
                 "version".blue(), "clear".blue(), "exit".blue());

        loop {
            print!(">>> ");
            io::stdout().flush()?;
            let mut line = String::new();
            let _ = io::stdin().read_line(&mut line)?;
            match line.trim() {
                "version" => println!("{} {}", Lox::name(), Lox::version()),
                "clear" => clearscreen::clear().unwrap_or_else(|e| {
                    eprintln!("{}: {e}", "error".red().bold());
                }),
                "exit" => return Ok(()),
                _ => self.run(line.into_bytes()).unwrap_or_else(|e| {
                    eprintln!("{}: {e}", "error".red().bold());
                }),
            }
        }
    }

    fn run(&self, source: Vec<u8>) -> Result<()> {
        let mut scanner = Scanner::new(source);
        for token in scanner.scan_tokens() {
            println!("{token:?}");
        }
        Ok(())
    }
}