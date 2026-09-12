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

    #[error("Lexer Error: {0}")]
    Lexer(#[from] LexerError),
}

#[derive(Error, Debug)]
pub enum LexerError
{
    #[error("{0} Is not a Identifier")]
    GrammarError(String),

    #[error("Unterminated String")]
    UnterminatedString,

    #[error("Too big number")]
    Overflow,

    #[error("Invalid simbol at {0}")]
    InvalidSimbol(usize),

    #[error("Find chars inside a number at {0}")]
    CorruptedNumber(usize),
}

#[derive(Error, Debug)]
pub enum ParserError
{
    #[error("Too many keywords!")]
    TooManyKeywords,

    #[error("{0}")]
    SyntaxError(String),

    #[error("Empty Input")]
    UnexpectedEOF,
}
