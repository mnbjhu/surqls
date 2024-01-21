use chumsky::span::SimpleSpan;

use super::lexer::Token;

pub type Span = SimpleSpan<usize>;

pub type Spanned<T> = (T, Span);

pub type ParserInput<'tokens, 'src> =
    chumsky::input::SpannedInput<Token, Span, &'tokens [(Token, Span)]>;
