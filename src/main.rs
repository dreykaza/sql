mod core;
use crate::core::interface;
mod compiler;
mod error;
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
