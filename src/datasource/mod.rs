mod directory;
mod file;
mod fixed;

pub use self::directory::*;
pub use self::file::*;
pub use self::fixed::*;

pub type Data = Vec<String>;

pub trait DataSource: Send {
  fn iter(self) -> Data;
}
