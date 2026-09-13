use crate::compiler::types::token::{Expression, Grammar, Keyword, Token};

pub struct Keywords;

impl<'a> Keywords
{
    pub fn lookup(word: &'a str) -> Token<'a>
    {
        let lower = word.to_ascii_lowercase();

        match lower.as_str()
        {
            "insert" => Token::Keyword(Keyword::Insert),
            "select" => Token::Keyword(Keyword::Select),
            "from" => Token::Expression(Expression::From),
            "column" => Token::Expression(Expression::Column),
            _ => Token::Identifier(word),
        }
    }
}

pub struct Grammer;

impl<'a> Grammer
{
    pub fn lookup(char: char) -> Token<'a>
    {
        match char
        {
            '(' => Token::Grammar(Grammar::OpenBracket),
            ')' => Token::Grammar(Grammar::CloaseBracket),
            ',' => Token::Grammar(Grammar::Comma),
            _ => unreachable!(),
        }
    }
}
