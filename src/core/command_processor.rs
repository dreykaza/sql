use crate::compiler;
use crate::error::{AppError, InterfaceError};
use std::process;

pub fn process_command(command: &str) -> Result<(), AppError>
{
    let instruction = command.split_whitespace().next().unwrap();

    if command.starts_with('.')
    {
        meta_commands(&command)?;
    }

    match instruction
    {
        // "insert" => compiler::insert_querry(command)?,
        // "select" => compiler::select_querry(command)?,
        _ => Err(InterfaceError::InvalidInput(instruction.to_string()))?,
    }
}

fn meta_commands(command: &str) -> Result<(), InterfaceError>
{
    match command
    {
        ".exit" => process::exit(0),
        _ => Err(InterfaceError::InvalidInput(command.to_string())),
    }
}
