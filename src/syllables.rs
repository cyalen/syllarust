use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut file = File::open("../cmudict/cmudict.dict")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("{}", contents);
    Ok(())
}