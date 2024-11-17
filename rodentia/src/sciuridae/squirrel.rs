#[derive(Debug)]
pub struct PrairieDog {
    pub legs: i32,
    name: String, // still private
}

impl PrairieDog {
    pub fn new_pdog(mut self, name: &str) {
        self.name = String::from(name);
        self.legs = 4;
    }
}

// all fileds are public
pub enum FurColor {
    Red,
    Yellow,
    Gray,
    Black,
}
mod squirrel {
    pub mod eurasian_red_squirrel {
        pub fn drinking_water() {}
    }
}

// doesn't work for interactions
use crate::sciuridae::squirrel::squirrel::eurasian_red_squirrel;

mod interactions {
    use super::squirrel::eurasian_red_squirrel;

    pub fn hydrating_a_red() {
        eurasian_red_squirrel::drinking_water();
    }
}
