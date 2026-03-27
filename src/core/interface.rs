use std::{
    fmt,
    io::{self, Write},
};

use crate::core::command_processor;

#[derive(Debug)]
pub enum Error
{
    InvalidInput(String),
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self
        {
            Error::InvalidInput(msg) => writeln!(f, "{}", msg),
        }
    }
}

pub fn read_line() -> Result<(), Error>
{
    print!("db > ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();

    if let Err(e) = io::stdin().read_line(&mut buffer)
    {
        return Err(Error::InvalidInput(e.to_string()));
    }

    command_processor::process_command(buffer.trim());

    Ok(())
}
