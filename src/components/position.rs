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

#[allow(dead_code)]
impl FloatPosition {
    pub fn to_int_position(&self) -> IntPosition {
        return IntPosition {
            x: self.x as i32,
            y: self.y as i32
        }
    }

    pub fn from_int(x: i32, y: i32) -> FloatPosition {
        FloatPosition { x: x as f32, y: y as f32 }
    }
}

