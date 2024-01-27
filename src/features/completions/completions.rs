use chumsky::{error::RichPattern, prelude::Input, util::Maybe, Parser};
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, CompletionParams};

use crate::{
    features::completions::has_completions::HasCompletionItems,
    lexer::{keyword::Keyword, lexer::lexer, token::Token},
    ls::backend::Backend,
    parser::parser::parser,
    util::range::span_to_range,
};

pub async fn get_completions(backend: &Backend, _params: CompletionParams) -> Vec<CompletionItem> {
    let mut completions = vec![];
    let uri = _params.text_document_position.text_document.uri;
    let rope = backend.document_map.get(uri.to_string().as_str()).unwrap();
    let text = rope.value().to_string();
    let (tokens, _) = lexer().parse(text.as_str()).into_output_errors();
    if let Some(tokens) = tokens {
        let mut scoped_items = backend.state.lock().await;
        let parser_result = parser().parse_with_state(
            tokens
                .as_slice()
                .spanned((rope.len_chars()..rope.len_chars()).into()),
            &mut scoped_items,
        );
        let (ast, parse_errs) = parser_result.into_output_errors();
        for err in parse_errs {
            let range = span_to_range(err.span(), &rope).unwrap();
            if range.start <= _params.text_document_position.position
                && _params.text_document_position.position <= range.end
            {
                for exp in err.expected() {
                    match exp {
                        RichPattern::Token(kw) => {
                            if let Maybe::Val(Token::Keyword(kw)) = kw {
                                completions.push(CompletionItem {
                                    label: kw.to_string(),
                                    kind: Some(CompletionItemKind::KEYWORD),
                                    ..Default::default()
                                });
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
        if let Some(ast) = ast {
            completions.extend(ast.get_completion_items(
                &mut scoped_items,
                _params.text_document_position.position,
                &rope,
            ))
        };
    }
    completions
}
