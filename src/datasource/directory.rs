use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;

pub struct Directory {
  files: Vec<File>,
}

impl Directory {
  pub fn new<T: AsRef<Path>>(path: T) -> Directory {
    match fs::read_dir(path) {
      Err(_) => Directory { files: vec![] },
      Ok(files) => {
        let files = files.filter_map(Result::ok).map(|file| File::open(file.path())).filter_map(Result::ok).collect();

        Directory { files }
      }
    }
  }
}

impl super::DataSource for Directory {
  fn iter(self) -> super::Data {
    self.files.iter().flat_map(|file| BufReader::new(file).lines().filter_map(Result::ok)).collect()

    // Some(file) => BufReader::new(file).lines().filter_map(Result::ok).collect(),
  }
}
