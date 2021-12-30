use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let data = &bs63::decode(input.trim()).into_vec()?;
    io::stdout().write_all(data)?;
    Ok(())
}
