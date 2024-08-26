use std::fs::File;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let mut file = File::open("text.txt")?;
    let mut buffer = [0; 10];

    let n = file.read(&mut buffer)?;

    println!("The byte: {:?}", &buffer[..n]);
    Ok(())
}
