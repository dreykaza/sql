use std::process;

use crate::compiler;

pub fn process_command(command: &str)
{
    let instruction = command.split_whitespace().next().unwrap();

    if command.starts_with('.')
    {
        meta_commands(&command);
    }

    if command.split_whitespace().count() < 3
    {
        println!("Not enoght arguments");
        return;
    }

    if instruction == "insert"
    {}

    if instruction == "select"
    {
        compiler::select_querry(command);
    }
}

fn meta_commands(command: &str)
{
    match command
    {
        ".exit" => process::exit(0),
        _ => println!("Not a command!"),
    }
}
