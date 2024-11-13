use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    /// Input file
    #[arg(required = true)]
    pub input: PathBuf,

    /// Output file
    #[arg(short, long, default_value = "output.png")]
    pub output: String,

    /// Width of the output image
    #[arg(short('W'), long, default_value = "1000")]
    pub width: u32,

    /// Height of the output image
    #[arg(short('H'), long, default_value = "1000")]
    pub height: u32,

    /// Print what is happening
    #[arg(short, long)]
    pub verbose: bool,

    /// Print the generated AST
    #[arg(long("ast"))]
    pub print_ast: bool,
}
