use clap::Parser;
use cli::command::{CliCommand, SubCommand};
use ls::server::launch_server;

use crate::cli::lex::lex;

mod ast;
mod cli;
mod declarations;
mod features;
mod lexer;
mod ls;
mod parser;
mod util;

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
