mod core;
use crate::core::interface;
mod compiler;
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
