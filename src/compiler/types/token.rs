#[derive(PartialEq, Debug)]
pub enum Keyword<'a>
{
    Select,
    Insert,
    Identifier(&'a str),
}

#[derive(PartialEq, Debug)]
pub enum Value<'a>
{
    String(&'a str),
    Number(i32),
}

#[derive(PartialEq, Debug)]
pub enum Grammar
{
    From,
    Column,
}

#[derive(PartialEq, Debug)]
pub enum Token<'a>
{
    Keyword(Keyword<'a>),
    Grammar(Grammar),
    Value(Value<'a>),
    EOF,
}
