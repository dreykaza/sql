use crate::core::command_processor;
use crate::error::{AppError, InterfaceError};
use std::io::{self, Write};

pub fn read_line()
{
    print!("db > ");
    io::stdout().flush().unwrap();
    let mut buffer = String::new();

    if let Err(e) = io::stdin().read_line(&mut buffer)
    {
        error_handler(AppError::InterfaceError(InterfaceError::InvalidInput(
            e.to_string(),
        )));
        return;
    }

    if buffer.trim().is_empty()
    {
        error_handler(AppError::InterfaceError(InterfaceError::EmptyInput));
        return;
    }

    if let Err(e) = command_processor::process_command(buffer.trim())
    {
        error_handler(e);
    }
}

fn error_handler(error: AppError)
{
    println!("{}", error)
}
