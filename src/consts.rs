#![allow(unused)]

pub mod exitcode {
    pub const OK: i32          = 0;
    pub const GENERIC_ERR: i32 = 1;
    pub const USAGE_ERR: i32   = 2; // `clap` uses exit code 2 for usage error.
    pub const IO_ERR: i32      = 3;
}