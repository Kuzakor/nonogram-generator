use core::fmt::Debug;

#[derive(Default, Debug)]
pub struct Crop {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32,
}

impl Crop {
    pub fn width(&self) -> u32 {
        self.right - self.left + 1
    }

    pub fn height(&self) -> u32 {
        self.bottom - self.top + 1
    }
}
