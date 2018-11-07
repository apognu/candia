pub struct Array {
  vec: super::Data,
}

impl Array {
  pub fn new(vec: &[String]) -> Array {
    Array { vec: vec.to_vec() }
  }
}

impl super::DataSource for Array {
  fn iter(self) -> super::Data {
    self.vec
  }
}
