use crate::core::command_processor;
use crate::error::InterfaceError;
use std::io::{self, Write};

pub fn read_line() -> Result<(), InterfaceError>
{
    print!("db > ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();

    if let Err(e) = io::stdin().read_line(&mut buffer)
    {
        return Err(InterfaceError::InvalidInput(e.to_string()));
    }

    if buffer.trim().is_empty()
    {
        return Err(InterfaceError::EmptyInput);
    }

    command_processor::process_command(buffer.trim())?;

    Ok(())
}
