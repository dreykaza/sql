#[derive(PartialEq)]
pub enum Keyword
{
    Select,
    Insert,
}

#[derive(PartialEq)]
pub enum Value
{
    String(String),
    Number(i32),
}

#[derive(PartialEq)]
pub enum Token
{
    Keyword(Keyword),
    Value(Value),
    EOF,
}
