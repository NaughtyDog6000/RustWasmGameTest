use specs::prelude::*;

use crate::{components::{left_mover::LeftMover, position::FloatPosition}, LastTickInstant};

pub struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (
        Read<'a, LastTickInstant>,
        ReadStorage<'a, LeftMover>, 
        WriteStorage<'a, FloatPosition>);

    fn run(&mut self, (last_tick, lefty, mut pos) : Self::SystemData) {
        let delta_time_s = web_time::Instant::now().duration_since(last_tick.0).as_secs_f32();
        // println!("delta: {}", delta_time_s);
        for (_lefty,pos) in (&lefty, &mut pos).join() {
            pos.x += _lefty.x_velocity * delta_time_s;
            pos.y += _lefty.y_velocity * delta_time_s;
        }
    }
}