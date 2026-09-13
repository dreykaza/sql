use crate::compiler::types::token::{Grammar, Keyword, Value};

pub struct Tree<'a>
{
    pub keyword: Keyword,
    pub children: Vec<Child<'a>>,
}

struct From
{
    table: String,
}

struct Columns
{
    colums: Vec<String>,
}

pub enum Child<'a>
{
    From(Grammar),
    Values(Vec<Value<'a>>),
}
