use crate::compiler::types::token::{Keyword, Token, Value};

pub fn tokenize(command: &str) -> Vec<Token>
{
    let mut tokens: Vec<Token> = vec![];

    for part in command.split_whitespace()
    {
        match part
        {
            "insert" => tokens.push(Token::Keyword(Keyword::Insert)),
            "select" => tokens.push(Token::Keyword(Keyword::Select)),
            _ =>
            {
                match part.parse()
                {
                    Ok(n) => tokens.push(Token::Value(Value::Number(n))),
                    Err(_) => tokens.push(Token::Value(Value::String(part.to_string()))),
                };
            }
        }
    }

    tokens.push(Token::EOF);

    tokens
}
