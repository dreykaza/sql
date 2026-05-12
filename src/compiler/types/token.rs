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
pub enum Grammar
{
    From,
    Column,
}

#[derive(PartialEq)]
pub enum Token
{
    Keyword(Keyword),
    Grammar(Grammar),
    Value(Value),
    EOF,
}
