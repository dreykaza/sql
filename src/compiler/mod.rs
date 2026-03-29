use crate::error::AppError;
mod code_generator;
mod parser;
mod tokenizer;

pub fn select_querry(command: &str) -> Result<(), AppError>
{
    let tokens = tokenizer::tokenize(command);

    Ok(())
}

pub fn insert_querry(command: &str) -> Result<(), AppError>
{
    let tokens = tokenizer::tokenize(command);

    Ok(())
}
