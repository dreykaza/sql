mod core;
use crate::core::interface;

fn main()
{
    loop
    {
        if let Err(e) = interface::read_line()
        {
            println!("{}", e);
        }
    }
}

