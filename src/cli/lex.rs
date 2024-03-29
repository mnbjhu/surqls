use std::{fs, ops::Range, path::PathBuf};

use chumsky::Parser;
use colored::{ColoredString, Colorize};

use crate::lexer::{lexer::lexer, token::Token};

pub fn lex(file: PathBuf) {
    let text = fs::read_to_string(file).unwrap();
    let rope = ropey::Rope::from_str(&text);
    let (tokens, _) = lexer().parse(&text).into_output_errors();
    match tokens {
        Some(tokens) => {
            let mut prev = 0;
            for (token, span) in tokens {
                if span.start > prev {
                    let range = Range {
                        start: prev,
                        end: span.start,
                    };
                    let text = rope.get_slice(range).unwrap();
                    print!("{}", text);
                }
                prev = span.end;
                let color = match token {
                    Token::Identifier(_) => "green",
                    Token::Keyword(_) => "yellow",
                    Token::Operator(_) => "magenta",
                    Token::Punctuation(_) => "cyan",
                    Token::String(_) => "blue",
                    Token::DateTime(_) => "red",
                    Token::Duration(_) => "red",
                    Token::Float(_) => "red",
                    Token::Integer(_) => "red",
                    Token::Decimal(_) => "red",
                    Token::RecordString(_) => "red",
                    Token::Boolean(_) => "yellow",
                    Token::Variable(_) => "white",
                    Token::Newline => "white",
                };
                let color = colored::Color::from(color);
                let mut text = rope
                    .get_slice(Range {
                        start: span.start,
                        end: span.end,
                    })
                    .unwrap()
                    .to_string();
                if token == Token::Newline {
                    text = format!("{}{}", "↵", text);
                }
                let text = ColoredString::from(text).color(color);
                print!("{}", text);
            }
        }
        None => {
            println!("No tokens found");
        }
    }
}
