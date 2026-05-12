use crate::{
    compiler::{
        lexer::lookup_table::Keywords,
        types::token::{Token, Value},
    },
    error::LexerError,
};
use std::iter::Peekable;
use std::str::Chars;

mod lookup_table;

struct Lexer;

impl Lexer
{
    pub fn tokenize(&mut self, input: String) -> Result<Vec<Token>, LexerError>
    {
        let mut tokens: Vec<Token> = vec![];
        let mut iter = input.chars().peekable();

        while let Some(&ch) = iter.peek()
        {
            match ch
            {
                '"' => tokens.push(Token::Value(Value::String(Self::read_string(&mut iter)?))),
                '0'..='9' => tokens.push(Token::Value(Value::Number(
                    Self::read_number(&mut iter)?.parse().unwrap(),
                ))),
                'a'..='z' | 'A'..='Z' => tokens.push(Self::read_identifier(&mut iter)?),
                _ =>
                {
                    iter.next();
                }
            }
        }

        tokens.push(Token::EOF);

        Ok(tokens)
    }

    fn read_identifier(iter: &mut Peekable<Chars>) -> Result<Token, LexerError>
    {
        let mut buffer = String::new();

        loop
        {
            match iter.next()
            {
                Some(' ') | None => return Keywords::lookup(&buffer),
                Some(ch @ 'a'..='z' | ch @ 'A'..='Z') => buffer.push(ch),
                Some(_) => return Err(LexerError::CorruptedIdentifier),
            }
        }
    }

    fn read_number(iter: &mut Peekable<Chars>) -> Result<String, LexerError>
    {
        let mut buffer = String::new();

        loop
        {
            match iter.next()
            {
                Some(' ') | None => return Ok(buffer),
                Some(ch @ '0'..='9') => buffer.push(ch),
                Some(_) => return Err(LexerError::CorruptedNumber),
            }
        }
    }

    fn read_string(iter: &mut Peekable<Chars>) -> Result<String, LexerError>
    {
        iter.next();
        let mut buffer = String::new();

        loop
        {
            match iter.next()
            {
                Some('"') => return Ok(buffer),
                Some(ch) => buffer.push(ch),
                None => return Err(LexerError::UnterminatedString),
            }
        }
    }
}
