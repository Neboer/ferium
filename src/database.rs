use std::env;
use std::ffi::{OsString};
use std::fs::File;
use std::io::{self, Write};

pub struct Database {
    pub(crate) file: Option<File>,
    pub file_path: Option<OsString>
}

impl Database {
    pub fn create_database() -> Result<Self, io::Error> {
        // 检查环境变量是否存在
        if let Some(db_path) = env::var_os("FERIUM_DB_PATH") {
            let file = File::create(&db_path)?;
            Ok(Database { file: Some(file), file_path: Some(db_path) })
        } else {
            Ok(Database { file: None, file_path: None})
        }
    }

    pub fn add_data(&mut self, data: [String; 2]) -> io::Result<()>
    {
        if let Some(ref mut file) = self.file {
            writeln!(file, "{}\t{}", data[0], data[1])?;
        }
        Ok(())
    }

    pub fn close_database(&mut self) -> io::Result<()> {
        // 关闭文件
        if let Some(file) = self.file.take() {
            file.sync_all()?;
        }
        Ok(())
    }
}
