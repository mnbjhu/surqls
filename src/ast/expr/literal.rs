#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    DateTime(String),
    Duration(String),
    RecordString(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Null,
}
