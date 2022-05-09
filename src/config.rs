use clap::Parser;
use std::fmt::{self, Display};

#[derive(Parser, Debug)]
#[clap(version, author, about)]
pub struct Config {
    #[clap(short = 'l', long = "log", help = "change log level", possible_values = &["trace", "debug", "info", "warn", "error", "off"])]
    #[cfg_attr(not(debug_assertions), clap(default_value = "warn"))]
    #[cfg_attr(debug_assertions, clap(default_value = "debug"))]
    pub log_level: log::LevelFilter,

    #[clap(help = "rom file to be loaded by the gameboy")]
    pub rom: Option<String>,

    #[clap(
        long = "breakpoint",
        short = 'b',
        help = "create and enable breakpoints at the start of the rom\n\
        breakpoints must be specified in the following format:\n\
        ./gbmu -b \"PC == 0050\" -b \"AF == 0010\" ...",
        multiple_occurrences = true,
        multiple_values = false,
        requires = "rom"
    )]
    pub breakpoints: Vec<String>,
    #[clap(
        long = "debug",
        short = 'd',
        help = "enable debug mode at the start of the rom",
        requires = "rom"
    )]
    pub debug: bool,

    #[clap(
        arg_enum,
        short = 'm',
        long,
        help = "force gameboy mode between color and mono"
    )]
    pub mode: Option<Mode>,
}

#[derive(Debug, clap::ArgEnum, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Color,
    Classic,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Color => write!(f, "color"),
            Mode::Classic => write!(f, "classic"),
        }
    }
}
