#[derive(PartialEq)]
pub enum Keyword
{
    Select,
    Insert,
}

#[derive(PartialEq)]
pub enum Value
{
    Email(String),
    Time(i32),
}

#[derive(PartialEq)]
pub enum Token
{
    Keyword(Keyword),
    Value(Value),
}
