use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {
    #[arg(required = true)]
    pub input: PathBuf,

    #[arg(short, long, default_value = "output.png")]
    pub output: String,

    #[arg(short('W'), long, default_value = "1000")]
    pub width: u32,

    #[arg(short('H'), long, default_value = "1000")]
    pub height: u32,

    /// Print the generated AST
    #[arg(long("ast"))]
    pub print_ast: bool,
}
