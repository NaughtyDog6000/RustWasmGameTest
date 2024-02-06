use specs::{prelude::*, Component};

#[derive(Component)]
pub struct Velocity {
    pub x_velocity: f32,
    pub y_velocity: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Velocity {
            x_velocity: 1.0,
            y_velocity: 2.0,
        }
    }
}

