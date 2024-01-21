use ropey::Rope;
use tower_lsp::lsp_types::Position;

pub fn offset_to_position(offset: usize, rope: &Rope) -> Option<Position> {
    let line = rope.try_char_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let column = offset - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}

pub fn position_to_offset(position: Position, rope: &Rope) -> Option<usize> {
    let line = position.line as usize;
    let column = position.character as usize;
    let offset = rope.try_line_to_char(line);
    match offset {
        Ok(offset) => Some(offset + column),
        Err(_) => None,
    }
}
