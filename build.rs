use serde::Deserialize;
use serde_json;
use std::convert::AsRef;
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Deserialize)]
struct EntryFields {
    title: String,
    pos: String,
    content: String,
}

fn read_entries<P: AsRef<Path>>(json_path: P) -> io::Result<Vec<EntryFields>> {
    let file_handle = File::open(json_path)?;
    let reader = io::BufReader::new(file_handle);
    let data: Vec<EntryFields> = serde_json::from_reader(reader)?;

    Ok(data)
}

fn main() -> io::Result<()> {
    let res = read_entries("jargon.json")?;

    for r in res {
        println!("{}", r.title);
    }

    Ok(())
}
