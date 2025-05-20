// examples/greeter.rs
use effective_rust::item30::greet;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let name = std::env::args().nth(1).unwrap_or_else(|| "World".into());
    println!("{}", greet(&name));
    Ok(())
}
