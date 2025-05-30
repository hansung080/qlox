#![allow(unused)]

pub mod exitcode {
    pub const OK: i32          = 0;
    pub const GENERIC_ERR: i32 = 1;
    pub const USAGE_ERR: i32   = 2; // `clap` uses exit code 2 for usage error.
    pub const IO_ERR: i32      = 3;
    pub const SCANNER_ERR: i32 = 50;
}

pub mod tag {
    pub const ERROR: &'static str = "\x1b[1;31merror\x1b[0m"; // red bold
}