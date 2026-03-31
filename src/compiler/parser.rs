use crate::{
    compiler::token::{Keyword, Token},
    error::ParserError,
};

fn select_parse(tokens: Vec<Token>) {}

fn keyword_number_check(tokens: &Vec<Token>) -> Result<(), ParserError>
{
    if tokens
        .iter()
        .filter(|x| {
            *x == &Token::Keyword(Keyword::Select) || *x == &Token::Keyword(Keyword::Insert)
        })
        .count()
        > 1
    {
        return Err(ParserError::TooManyKeywords);
    }

    Ok(())
}

// if command.split_whitespace().count() < 3
// {
//     println!("Not enoght arguments");
//     return;
// }
