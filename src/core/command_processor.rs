use std::process;

pub fn process_command(command: &str)
{
    if command.starts_with('.')
    {
        meta_commands(&command);
    }

    let mut instruction = command.split_whitespace();

    if command.split_whitespace().count() < 3
    {
        println!("Not enoght arguments");
    }

    if instruction.next().unwrap() == "insert"
    {
        println!("inserting {}", instruction.next().unwrap());
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
