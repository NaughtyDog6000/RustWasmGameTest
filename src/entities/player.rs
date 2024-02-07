use bracket_terminal::{
    console,
    prelude::{BTerm, VirtualKeyCode, RGB},
};
use specs::{prelude::*, Component};

use crate::{Renderable, State};

use crate::FloatPosition;

#[derive(Component, Debug)]
pub struct Player {}

pub fn move_player(delta_x: f32, delta_y: f32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<FloatPosition>();
    let mut players = ecs.write_storage::<Player>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        pos.x += delta_x;
        pos.y += delta_y;
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => move_player(-1.0, 0.0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => move_player(1.0, 0.0, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => move_player(0.0, -1.0, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => move_player(0.0, 1.0, &mut gs.ecs),
            VirtualKeyCode::Comma => console::log("heelloo"),
            _ => {}
        },
    }
}
