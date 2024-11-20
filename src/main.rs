mod nbt_reader;
mod nbt_value;
mod tags;

use std::env;
use std::fs::File;
use std::io::{self, Cursor, Write};
use nbt_reader::NbtReader;

fn write_file(path: &str, text: &str) -> io::Result<()> {
    let mut buffer = File::create(path)?;
    buffer.write_all(text.as_bytes())
}

fn main() -> io::Result<()> {
    let path = env::args().nth(1).expect("Please provide the path of the nbt file. Usage: cargo run file.nbt");
    let output = "output.json";

    let reader = NbtReader::new();

    let data = reader.read_nbt_file(&path);
    let mut cursor = Cursor::new(data.unwrap());

    if let Ok(value) = reader.parse_nbt(&mut cursor) {
        let serialized = serde_json::to_string_pretty(&value).unwrap();
        write_file(output, &serialized)?;

        println!("Converted successfully in {output}");
    } else {
        panic!("Failed to parse the NBT file");
    }

    Ok(())
}