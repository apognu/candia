use std::fs::File as OsFile;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct File {
  file: Option<OsFile>,
}

impl File {
  pub fn new<T: AsRef<Path>>(path: T) -> File {
    match OsFile::open(path) {
      Err(_) => File { file: None },
      Ok(file) => File { file: Some(file) },
    }
  }
}

impl super::DataSource for File {
  fn iter(self) -> super::Data {
    match self.file {
      None => vec![],
      Some(file) => BufReader::new(file).lines().filter_map(Result::ok).collect(),
    }
  }
}
