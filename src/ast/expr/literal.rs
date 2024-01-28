#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    DateTime(String),
    Duration(String),
    RecordString(String),
    Int(String),
    Float(String),
    Decimal(String),
    Bool(bool),
    Null,
}
