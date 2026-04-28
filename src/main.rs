mod core;
use crate::core::interface;
mod compiler;
mod error;

fn main()
{
    loop
    {
        interface::read_line();
    }
}
