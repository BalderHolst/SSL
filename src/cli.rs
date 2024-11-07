use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Cli {

    #[arg(required = true)]
    pub input: PathBuf,

}
