use crate::error::CompilerError;
mod code_generator;
mod lexer;
mod parser;
mod token;

pub fn select_querry(command: &str) -> Result<(), CompilerError>
{
    let tokens = lexer::tokenize(command);

    Ok(())
}

pub fn insert_querry(command: &str) -> Result<(), CompilerError>
{
    let tokens = lexer::tokenize(command);

    Ok(())
}
