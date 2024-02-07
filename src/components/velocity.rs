use specs::{prelude::*, Component};
use serde::{Deserialize, Serialize};

#[derive(Component, Clone, Copy, Debug, Deserialize, Serialize)]
pub struct Velocity {
    pub x_velocity: f32,
    pub y_velocity: f32,
}

#[allow(dead_code)]
impl Velocity {
    pub fn default() -> Self {
        Velocity {
            x_velocity: 0.0,
            y_velocity: 0.0,
        }
    }
    pub fn from_f32(x: f32, y: f32) -> Self {
        Velocity {
            x_velocity: x,
            y_velocity: y,
        }
    }
    pub fn from_i32(x: i32, y: i32) -> Self {
        Velocity {
            x_velocity: x as f32,
            y_velocity: y as f32,
        }
    }

}
