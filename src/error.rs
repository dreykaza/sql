use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError
{
    #[error("Compiler Error: {0}")]
    CompilerError(#[from] CompilerError),

    #[error("Interface Error: {0}")]
    InterfaceError(#[from] InterfaceError),
}

#[derive(Error, Debug)]
pub enum InterfaceError
{
    #[error("Not a command! {0}")]
    InvalidInput(String),

    #[error("Input is empty!")]
    EmptyInput,
}

#[derive(Error, Debug)]
pub enum CompilerError
{
    #[error("Parser Error: {0}")]
    Parser(#[from] ParserError),
}

#[derive(Error, Debug)]
pub enum ParserError
{
    #[error("Too many keywords!")]
    TooManyKeywords,
}
