use crate::{
    compiler::types::token::{Grammar, Keyword, Token},
    error::LexerError,
};

pub struct Keywords;

impl Keywords
{
    pub fn lookup(word: &str) -> Result<Token, LexerError>
    {
        match word
        {
            "insert" => Ok(Token::Keyword(Keyword::Insert)),
            "select" => Ok(Token::Keyword(Keyword::Select)),
            "from" => Ok(Token::Grammar(Grammar::From)),
            "column" => Ok(Token::Grammar(Grammar::Column)),
            _ => Err(LexerError::GrammarError(word.to_string())),
        }
    }
}
