use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError
{
    #[error("Interface Error: {0}!")]
    Interface(#[from] InterfaceError),

    #[error("Parser Error: {0}!")]
    Parser(#[from] ParserError),
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
pub enum ParserError
{
    #[error("data store disconnected")]
    TooManyKeywords,
}
