use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct Database {
    pub file: File,
    pub file_path: PathBuf,
}

impl Database {
    pub fn new(path: PathBuf) -> Result<Self, io::Error> {
        let file = File::create(&path)?;
        Ok(Database {
            file,
            file_path: path,
        })
    }

    pub fn add_data(&mut self, data: [String; 2]) -> io::Result<()> {
        writeln!(&mut self.file, "{}\t{}", data[0], data[1])?;
        Ok(())
    }

    pub fn close_database(self) -> io::Result<()> {
        self.file.sync_all()?;
        Ok(())
    }
}
