use std::{fs, io};

fn main() -> io::Result<()> {
    let files = fs::read_dir(".")?
        .map(|res| res.map(|e| e.file_name()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for file in files {
        let filename = file.into_string().unwrap();
        print!("{} ", filename);
    }

    Ok(())
}
