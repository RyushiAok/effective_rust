// examples/greeter.rs
use effective_rust::item30::add ;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {  
    let a: i32 = 1;
    let b: i32 = 2; 
    let result = add(a, b);
    println!("The sum of {} and {} is {}", a, b, result);
    Ok(())
}
