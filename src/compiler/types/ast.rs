use crate::compiler::types::token::{Keyword, Value};

struct Tree
{
    keyword: Keyword,
    children: Vec<Child>,
}

enum Child
{
    From(String),
    Column(Vec<String>),
    Values(Vec<Value>),
}
