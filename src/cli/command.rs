use std::path::PathBuf;

use clap::Parser;

/// SurrealDB Language Server
#[derive(Debug, Parser)]
pub struct CliCommand {
    #[clap(subcommand)]
    pub subcommand: Option<SubCommand>,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    /// Lex a file
    #[clap(name = "lex")]
    Lex {
        #[clap(short, long)]
        file: PathBuf,
    },
}
