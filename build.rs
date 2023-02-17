use serde::Deserialize;
use serde_json;
use std::convert::AsRef;
use std::format;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Deserialize)]
struct EntryFields {
    title: String,
    pos: String,
    content: String,
}

impl EntryFields {
    fn expand_code(&self) -> String {
        /* format!(
            "make_entry!(r\"{}\",\"{}\",r#\"{}\"#),\n",
            self.title, self.pos, self.content
        )*/

        format!(
            "[\nr\"{}\", \"{}\",r#####\"{}\"#####\n],\n\n",
            self.title, self.pos, self.content
        )
    }
}

fn read_entries<P: AsRef<Path>>(json_path: P) -> std::io::Result<Vec<EntryFields>> {
    let file_handle = File::open(json_path)?;
    let reader = std::io::BufReader::new(file_handle);
    let data: Vec<EntryFields> = serde_json::from_reader(reader)?;

    Ok(data)
}

fn write_to_file<P: AsRef<Path>>(
    entries: &Vec<EntryFields>,
    target_path: P,
) -> std::io::Result<()> {
    let entries_count = entries.len();
    let mut file_handle = File::create(&target_path)?;

    writeln!(&mut file_handle, "//Macros? You mean format strings?\n")?;

    writeln!(
        &mut file_handle,
        "pub const NUM_ENTRIES:usize = {};",
        entries_count
    )?;

    writeln!(
        &mut file_handle,
        "pub const ENTRIES: [[&str; 3]; NUM_ENTRIES] = ["
    )?;

    for entry in entries {
        file_handle.write_all(entry.expand_code().as_bytes())?;
    }

    writeln!(&mut file_handle, "];")?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    let res = read_entries("src/jargon.json")?;

    write_to_file(&res, "src/gen_entries.rs")?;

    Ok(())
}
