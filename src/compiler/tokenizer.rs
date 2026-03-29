#[derive(PartialEq)]
pub enum Keyword
{
    Select,
    Insert,
}

#[derive(PartialEq)]
pub enum Value
{
    Email(String),
    Time(i32),
}

#[derive(PartialEq)]
pub enum Token
{
    Keyword(Keyword),
    Value(Value),
}

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
                    Ok(n) => tokens.push(Token::Value(Value::Time(n))),
                    Err(_) => tokens.push(Token::Value(Value::Email(part.to_string()))),
                };
            }
        }
    }

    tokens
}
