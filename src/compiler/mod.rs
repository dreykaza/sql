use crate::compiler::lexer::Lexer;
use crate::error::CompilerError;
mod code_generator;
mod lexer;
mod parser;
mod types;

pub fn select_querry(command: &str) -> Result<(), CompilerError>
{
    let mut lexer = Lexer::new(command);
    let tokens = lexer.tokenize();

    for n in tokens
    {
        println!("{n:?}")
    }
    Ok(())
}

pub fn insert_querry(command: &str) -> Result<(), CompilerError>
{
    // let tokens = lexer::tokenize(command);

    Ok(())
}
