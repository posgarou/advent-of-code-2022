#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Food {
  pub calories: i32,
}

impl Food {
  pub fn new(calories: i32) -> Food {
    Food { calories }
  }
}

impl From<i32> for Food {
  fn from(calories: i32) -> Self {
    Food { calories }
  }
}