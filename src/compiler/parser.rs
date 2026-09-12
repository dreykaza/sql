use crate::compiler::types::{
    ast::Tree,
    token::{Keyword, Token},
};
use crate::error::ParserError;
// use std::iter::Peekable;
// use std::str::Chars;

struct Parser;

impl Parser
{
    pub fn parse(tokens: Vec<Token>) -> Result<Tree, ParserError>
    {
        let mut iter = tokens.into_iter().peekable();
        let mut tree: Tree = Self::create_tree(iter.next())?;

        while let Some(ch) = iter.peek()
        {
            match ch
            {
                Token::Keyword(_) =>
                {
                    return Err(ParserError::TooManyKeywords);
                }
                Token::Value(v) =>
                {}
                Token::Grammar(g) =>
                {}
                Token::EOF =>
                {
                    break;
                }
            }
        }

        Ok(tree)
    }

    fn create_tree(token: Option<Token>) -> Result<Tree, ParserError>
    {
        let mut tree: Tree;

        match token
        {
            Some(Token::Keyword(keyword)) =>
            {
                tree = Tree {
                    keyword: keyword,
                    children: vec![],
                }
            }

            Some(token) =>
            {
                return Err(ParserError::SyntaxError(format!(
                    "{token:?} is not a Keyword"
                )))
            }

            None => return Err(ParserError::UnexpectedEOF),
        }

        Ok(tree)
    }
}
