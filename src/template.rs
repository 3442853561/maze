const RECT_MAX: usize = 200;
pub struct Rectangle {
    data: [[bool; RECT_MAX]; RECT_MAX],
    width: u8,
    height: u8,
    enter: (u8, u8),
    exit: (u8, u8),
}

impl Rectangle {
    pub fn new(width: u8, height: u8, enter: (u8, u8), exit: (u8, u8)) -> Result<Rectangle, String> {
        unimplemented!();
    }
}
