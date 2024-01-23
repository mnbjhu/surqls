use clap::Parser;
use cli::command::{CliCommand, SubCommand};
use ls::server::launch_server;

use crate::cli::lex::lex;

mod cli;
mod core;
mod features;
mod ls;

#[tokio::main]
async fn main() {
    match CliCommand::parse() {
        CliCommand {
            subcommand: Some(subcommand),
        } => match subcommand {
            SubCommand::Lex { file } => {
                println!("Lexing file: {:?}", file);
                lex(file);
            }
        },
        _ => {
            launch_server().await;
        }
    }
}
