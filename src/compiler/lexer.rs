use crate::{
    compiler::{
        lexer::lookup_table::Keywords,
        types::token::{Token, Value},
    },
    error::LexerError,
};
use std::cell::Cell;
use std::iter::Peekable;
use std::str::Chars;

mod lookup_table;

pub struct Lexer<'a>
{
    input: &'a str,
    pos: Cell<usize>,
}

impl<'a> Lexer<'a>
{
    pub fn new(input: &'a str) -> Self
    {
        Lexer {
            input: input,
            pos: 0.into(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'a>>, LexerError>
    {
        let mut tokens: Vec<Token> = vec![];
        let mut iter = self.input.chars().peekable();

        while let Some(&ch) = iter.peek()
        {
            match ch
            {
                '"' =>
                {
                    tokens.push(Token::Value(Value::String(Self::read_string(
                        self, &mut iter,
                    )?)));
                    self.advance();
                }
                '0'..='9' => match Self::read_number(self, &mut iter)?.parse::<i32>()
                {
                    Ok(n) => tokens.push(Token::Value(Value::Number(n))),
                    Err(_) => return Err(LexerError::Overflow),
                },
                'a'..='z' | 'A'..='Z' => tokens.push(Self::read_identifier(self, &mut iter)),
                ' ' =>
                {
                    iter.next();
                    self.advance();
                }
                _ => return Err(LexerError::InvalidSimbol(self.position())),
            }
        }

        tokens.push(Token::EOF);

        Ok(tokens)
    }

    fn advance(&self)
    {
        self.pos.set(self.pos.get() + 1);
    }

    fn position(&self) -> usize
    {
        self.pos.get()
    }

    fn read_identifier(&self, iter: &mut Peekable<Chars>) -> Token<'a>
    {
        let start = self.position();

        loop
        {
            match iter.peek()
            {
                Some('a'..='z' | 'A'..='Z' | '_' | '0'..='9') =>
                {
                    iter.next();
                }
                Some(_) | None => return Keywords::lookup(&self.input[start..self.position()]),
            }
            self.advance();
        }
    }

    fn read_number(&self, iter: &mut Peekable<Chars>) -> Result<&'a str, LexerError>
    {
        let start = self.position();

        loop
        {
            match iter.peek()
            {
                Some('0'..='9') =>
                {
                    iter.next();
                }
                Some('a'..='z' | 'A'..='Z') =>
                {
                    return Err(LexerError::CorruptedNumber(self.position()));
                }
                Some(_) | None => return Ok(&self.input[start..self.position()]),
            }
            self.advance();
        }
    }

    fn read_string(&self, iter: &mut Peekable<Chars>) -> Result<&'a str, LexerError>
    {
        iter.next();
        self.advance();
        let start = self.position();

        loop
        {
            match iter.next()
            {
                Some('"') => return Ok(&self.input[start..self.position()]),
                Some(_) =>
                {}
                None => return Err(LexerError::UnterminatedString),
            }
            self.advance();
        }
    }
}
