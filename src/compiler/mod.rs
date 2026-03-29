mod code_generator;
mod parser;
mod tokenizer;

pub fn select_querry(command: &str)
{
    let tokens = tokenizer::tokenize(command);
    for a in tokens
    {
        println!("token {:?}", a);
    }
}

pub fn insert_querry(command: &str)
{
    let tokens = tokenizer::tokenize(command);
    for a in tokens
    {
        println!("token {:?}", a);
    }
}
