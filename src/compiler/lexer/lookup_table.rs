use crate::compiler::types::token::{Grammar, Keyword, Token};

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
            "from" => Token::Grammar(Grammar::From),
            "column" => Token::Grammar(Grammar::Column),
            _ => Token::Keyword(Keyword::Identifier(word)),
        }
    }
}
