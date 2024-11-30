use std::{
    fs::File,
    io::{BufReader, Read},
};

fn main() -> anyhow::Result<()> {
    let file = File::open("index.txt");
    let mut _buffer = BufReader::new(file?);

    let mut buffer = vec![0; 200];
    _buffer.read_exact(&mut buffer)?;

    let str_buffer = String::from_utf8(buffer)?;

    println!("{:?}", str_buffer);
    Ok(())
}
