use chumsky::{primitive::just, recovery::via_parser, IterParser, Parser};
use ropey::Rope;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Diagnostic, DiagnosticSeverity, Position,
};

use crate::{
    core::{
        lexer::Token,
        parser::{
            delcarations::{ScopedItems, Type},
            parser::Extra,
        },
        span::{ParserInput, Spanned},
    },
    ls::util::range::span_to_range,
};

use super::{
    field::{field_parser, Field},
    newline::optional_new_line,
    parser::{Expression, HasCompletionItemsForType, HasDiagnosticsForType},
};

pub fn object_entry<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<ObjectEntry>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let entry = field_parser()
        .then_ignore(just(Token::Punctuation(':')).padded_by(optional_new_line()))
        .then(expr)
        .map_with(|(k, v), s| {
            (
                ObjectEntry {
                    key: k,
                    value: Some(v),
                },
                s.span(),
            )
        });
    entry.recover_with(via_parser(
        field_parser()
            .then_ignore(
                optional_new_line()
                    .then(just(Token::Punctuation(':')))
                    .or_not(),
            )
            .map_with(|k, s| {
                (
                    ObjectEntry {
                        key: k,
                        value: None,
                    },
                    s.span(),
                )
            }),
    ))
}

#[derive(Debug, Clone)]
pub struct ObjectEntry {
    pub key: Spanned<Field>,
    pub value: Option<Spanned<Expression>>,
}

pub fn object_parser<'tokens, 'src: 'tokens>(
    expr: impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
        + Clone
        + 'tokens,
) -> impl Parser<'tokens, ParserInput<'tokens, 'src>, Spanned<Expression>, Extra<'tokens>>
       + Clone
       + 'tokens {
    let object = object_entry(expr.clone())
        .separated_by(just(Token::Punctuation(',')).padded_by(optional_new_line()))
        .allow_trailing()
        .collect()
        .delimited_by(
            just(Token::Punctuation('{')).then(optional_new_line()),
            optional_new_line().then(just(Token::Punctuation('}'))),
        )
        .map(|v| Expression::Object(v))
        .map_with(|e, s| (e, s.span()))
        .boxed();
    object
}

impl HasCompletionItemsForType for Vec<Spanned<ObjectEntry>> {
    fn get_completion_items_for_type(
        &self,
        scope: &mut ScopedItems,
        position: Position,
        rope: &ropey::Rope,
        type_: &Type,
    ) -> Vec<CompletionItem> {
        let mut items = vec![];
        match type_ {
            Type::Object(obj) => {
                for (entry, span) in self {
                    let range = span_to_range(&span, rope).unwrap();
                    if range.start <= position && position <= range.end {
                        let ObjectEntry { key, value } = entry;
                        let key_range = span_to_range(&key.1, rope).unwrap();
                        let current = &self
                            .into_iter()
                            .map(|x| match &x.0.key.0 {
                                Field::Found(name, _) => name.to_string(),
                                Field::NotFound(name) => name.to_string(),
                            })
                            .collect::<Vec<_>>();
                        let mut missing = obj.fields.clone().into_iter().collect::<Vec<_>>();
                        missing.retain(|x| !current.contains(&x.name));
                        if key_range.start <= position && position <= key_range.end {
                            let cmps = missing
                                .iter()
                                .map(|f| CompletionItem {
                                    label: f.name.clone(),
                                    kind: Some(CompletionItemKind::FIELD),
                                    detail: Some(format!("{}", f.ty)),
                                    ..Default::default()
                                })
                                .collect::<Vec<_>>();
                            items.extend(cmps);
                        }
                        if let Some(value) = value {
                            let value_range = span_to_range(&value.1, rope).unwrap();
                            if value_range.start <= position && position <= value_range.end {
                                items.extend(
                                    value.0.get_completion_items_for_type(
                                        scope,
                                        position,
                                        rope,
                                        obj.get_field(&key.0.to_string())
                                            .map(|x| &x.ty)
                                            .unwrap_or(&Type::Any),
                                    ),
                                );
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        items
    }
}

impl HasDiagnosticsForType for Spanned<&Vec<Spanned<ObjectEntry>>> {
    fn diagnostics_for_type(
        &self,
        rope: &Rope,
        type_: &Type,
        scope: &mut ScopedItems,
    ) -> Vec<tower_lsp::lsp_types::Diagnostic> {
        match type_ {
            Type::Object(obj) => {
                let mut diagnostics = vec![];
                let current = &self
                    .0
                    .into_iter()
                    .map(|x| match &x.0.key.0 {
                        Field::Found(name, _) => name.to_string(),
                        Field::NotFound(name) => name.to_string(),
                    })
                    .collect::<Vec<_>>();
                let mut missing = obj.fields.clone().into_iter().collect::<Vec<_>>();
                missing.retain(|x| !current.contains(&x.name));
                if !missing.is_empty() {
                    let missing_text = missing
                        .iter()
                        .map(|x| x.name.clone())
                        .collect::<Vec<_>>()
                        .join(", ");
                    let diag = Diagnostic {
                        range: span_to_range(&self.1, rope).unwrap(),
                        severity: Some(DiagnosticSeverity::ERROR),
                        message: format!("Object is missing fields: {}", missing_text),
                        ..Default::default()
                    };
                    diagnostics.push(diag);
                }
                diagnostics
            }
            _ => {
                vec![]
            }
        }
    }
}
