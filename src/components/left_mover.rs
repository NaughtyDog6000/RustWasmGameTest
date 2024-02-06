use specs::{prelude::*, Component};

#[derive(Component)]
pub struct LeftMover {
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Default for LeftMover {
    fn default() -> Self {
        LeftMover {
            x_velocity: 1.0,
            y_velocity: 2.0,
        }
    }
}

