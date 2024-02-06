use specs::{prelude::*, Component};

#[derive(Component)]
pub struct IntPosition {
    pub x: i32,
    pub y: i32,
}
#[derive(Component)]
pub struct FloatPosition {
    pub x: f32,
    pub y: f32,
}

impl FloatPosition {
    pub fn to_int_position(&self) -> IntPosition {
        return IntPosition {
            x: self.x as i32,
            y: self.y as i32
        }
    }
}