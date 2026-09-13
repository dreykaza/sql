#[derive(PartialEq, Debug)]
pub enum Keyword
{
    Select,
    Insert,
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
    OpenBracket,
    CloaseBracket,
    Comma,
}

#[derive(PartialEq, Debug)]
pub enum Expression
{
    From,
    Column,
}

#[derive(PartialEq, Debug)]
pub enum Token<'a>
{
    Keyword(Keyword),
    Expression(Expression),
    Value(Value<'a>),
    Grammar(Grammar),
    Identifier(&'a str),
    EOF,
}
