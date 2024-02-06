use bracket_terminal::prelude::*;
use components::left_mover::LeftMover;
use specs::{prelude::*, Component};
use systems::left_walker::LeftWalker;
use web_time::Instant;

mod entities;
mod components;
mod systems;

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Renderable {
    glyph: bracket_terminal::FontCharType,
    fg: RGB,
    bg: RGB,
}
struct State {
    ecs: World,
    next_logic_tick: u128,
    start_instant: Instant,
    logic_tick_interval: u128,
}

impl State {
    fn run_systems(&mut self) {
        let mut lw = LeftWalker{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }

    fn new() -> State {
        State {
            ecs: World::new(),
            start_instant: web_time::Instant::now(),
            logic_tick_interval: 500,
            next_logic_tick: 0
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let now = self.start_instant.elapsed().as_millis();

        // clear the screen and set background
        ctx.cls_bg(RGB::from_f32(0.5, 0.5, 1.0));
        // display the framtime in the top left
        ctx.print(0, 0, ctx.frame_time_ms);

        // if it is a logic tick then run the logic systems
        if now > self.next_logic_tick {
            self.next_logic_tick = now + self.logic_tick_interval;
            self.run_systems();
        }

        ctx.print(1, 1, "Hello Bracket World");

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(160,90)?
        .with_title("Flappy birb Terminal")
        .with_fps_cap(144.0)
        .with_fullscreen(false)
        .build()?;

    let mut gs = State::new();
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();

    // Create Player
    gs.ecs.create_entity()
    .with(Position {x: 40, y: 25})
    .with(Renderable {
        glyph: to_cp437('P'),
        bg: RGB::from_u8(0,0,0),
        fg: RGB::from_u8(255, 255, 0),
    })
    .build();

    // create moving tubes
    for i in 0..10 {
        gs.ecs.create_entity()
        .with(Position { x: i * 6 + 20, y: 20 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::from_u8(0, 255, 0),
            bg: RGB::new(),
        })
        .with(LeftMover{})
        .build();
    }


    main_loop(context, gs)
}