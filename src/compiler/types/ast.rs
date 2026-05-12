use crate::compiler::types::token::{Grammar, Keyword, Value};

struct Tree
{
    keyword: Keyword,
    children: Vec<Child>,
}

struct From
{
    table: String,
}

struct Columns
{
    colums: Vec<String>,
}

enum Child
{
    From(Grammar),
    Values(Vec<Value>),
}
