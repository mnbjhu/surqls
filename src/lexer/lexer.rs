use chumsky::{
    extra,
    prelude::Rich,
    primitive::{choice, just, none_of, one_of},
    recovery::via_parser,
    text, IterParser, Parser,
};

use crate::util::span::Span;

use super::{keyword::Keyword, token::Token};

pub fn lexer<'a>() -> impl Parser<'a, &'a str, Vec<(Token, Span)>, extra::Err<Rich<'a, char>>> {
    let space = one_of(" \t").repeated().ignored().or_not();
    let ident = text::ident()
        .map(|s: &str| {
            let lower = s.to_lowercase();
            match lower.as_str() {
                "select" => Token::Keyword(Keyword::Select),
                "create" => Token::Keyword(Keyword::Create),
                "insert" => Token::Keyword(Keyword::Insert),
                "update" => Token::Keyword(Keyword::Update),
                "delete" => Token::Keyword(Keyword::Delete),
                "from" => Token::Keyword(Keyword::From),
                "where" => Token::Keyword(Keyword::Where),
                "and" => Token::Keyword(Keyword::And),
                "or" => Token::Keyword(Keyword::Or),
                "not" => Token::Keyword(Keyword::Not),
                "null" => Token::Keyword(Keyword::Null),
                "define" => Token::Keyword(Keyword::Define),
                "table" => Token::Keyword(Keyword::Table),
                "field" => Token::Keyword(Keyword::Field),
                "type" => Token::Keyword(Keyword::Type),
                "on" => Token::Keyword(Keyword::On),
                "as" => Token::Keyword(Keyword::As),
                "order" => Token::Keyword(Keyword::Order),
                "by" => Token::Keyword(Keyword::By),
                "limit" => Token::Keyword(Keyword::Limit),
                "skip" => Token::Keyword(Keyword::Skip),
                "content" => Token::Keyword(Keyword::Content),
                "return" => Token::Keyword(Keyword::Return),
                "full" => Token::Keyword(Keyword::Full),
                "none" => Token::Keyword(Keyword::None),
                "permissions" => Token::Keyword(Keyword::Permissions),
                "true" => Token::Boolean(true),
                "false" => Token::Boolean(false),
                _ => Token::Identifier(s.to_string()),
            }
        })
        .map_with(|t, s| (t, s.span()));

    let digits = text::digits(10).to_slice();

    let int = digits
        .clone()
        .map(|s: &str| Token::Integer(s.parse().unwrap()))
        .map_with(|t, e| (t, e.span()));

    let float = digits
        .clone()
        .then_ignore(just('.'))
        .then(digits.clone())
        .map(|(a, b): (&str, &str)| Token::Float(format!("{}.{}", a, b).parse().unwrap()))
        .map_with(|t, e| (t, e.span()));

    let escape = just('\\')
        .then(choice((
            just('\\'),
            just('/'),
            just('"'),
            just('b').to('\x08'),
            just('f').to('\x0C'),
            just('n').to('\n'),
            just('r').to('\r'),
            just('t').to('\t'),
            just('u').ignore_then(text::digits(16).exactly(4).to_slice().validate(
                |digits, e, emitter| {
                    char::from_u32(u32::from_str_radix(digits, 16).unwrap()).unwrap_or_else(|| {
                        emitter.emit(Rich::custom(e.span(), "invalid unicode character"));
                        '\u{FFFD}' // unicode replacement character
                    })
                },
            )),
        )))
        .ignored();

    let string = none_of("\\\"\n")
        .ignored()
        .or(escape)
        .repeated()
        .to_slice()
        .map(|s: &str| Token::String(s.to_string()))
        .delimited_by(
            just('"'),
            just('"')
                .ignored()
                .recover_with(via_parser(one_of("\n").ignored())),
        )
        .map_with(|t, e| (t, e.span()));

    let punctuation = one_of("#_.,;:{}[]()")
        .map(Token::Punctuation)
        .map_with(|t, e| (t, e.span()));

    let op = one_of("+-*/=<>|&!")
        .to_slice()
        .then(one_of("+-*/=<>|&!").repeated().collect::<String>())
        .map(|(f, s)| Token::Operator(format!("{}{}", f, s)))
        .map_with(|t, e| (t, e.span()));

    let new_line = one_of("\n").ignored().padded_by(space);

    let implicit_semi = new_line
        .clone()
        .then_ignore(new_line.repeated())
        .map_with(|_, e| (Token::Newline, e.span()));

    let semi = just(";")
        .map_with(|_, e| (Token::Punctuation(';'), e.span()))
        .padded_by(new_line.repeated());

    choice((
        semi,
        ident,
        int,
        float,
        string,
        punctuation,
        op,
        implicit_semi,
    ))
    .padded_by(space)
    .repeated()
    .collect::<Vec<_>>()
}
