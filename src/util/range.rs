use ropey::Rope;
use tower_lsp::lsp_types::Range;

use super::{offset::offset_to_position, span::Span};

pub fn span_to_range(span: &Span, rope: &Rope) -> Option<Range> {
    let start_position = offset_to_position(span.start, rope)?;
    let end_position = offset_to_position(span.end, rope)?;
    Some(Range::new(start_position, end_position))
}
