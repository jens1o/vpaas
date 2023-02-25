use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Dimensions {
    width: u32,
    height: u32,
}

impl Dimensions {
    pub fn new(width: u32, height: u32) -> Dimensions {
        Dimensions { width, height }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
