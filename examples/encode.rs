use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = Vec::<u8>::new();
    io::stdin().read_to_end(&mut input)?;
    println!("{}", bs63::encode(input).into_string());
    Ok(())
}
