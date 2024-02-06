use bracket_terminal::prelude::*;
use components::{player::{player_input, Player}, position::{FloatPosition, IntPosition}, velocity::Velocity};
use specs::{prelude::*, Component};
use specs::WorldExt;
use systems::velocity_movement::VelocityMovement;
use web_time::Instant;
mod entities;
mod components;
mod systems;


#[derive(Component)]
pub struct Renderable {
    glyph: bracket_terminal::FontCharType,
    fg: RGB,
    bg: RGB,
}


#[allow(dead_code)]
struct State {
    ecs: World,
    start_instant: Instant,
}



// used to figure out the deltatime in the next tick
pub struct LastTickInstant(Instant);

impl Default for LastTickInstant {
    fn default() -> Self {
        LastTickInstant(web_time::Instant::now())
    }
}


impl State {
    fn run_systems(&mut self) {
        let mut lw = VelocityMovement{};
        lw.run_now(&self.ecs);
        self.ecs.maintain();
    }

    
    fn new() -> State {
        State {
            ecs: World::new(),
            start_instant: web_time::Instant::now(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
       
        // clear the screen and set background
        ctx.cls_bg(RGB::from_f32(0.5, 0.5, 1.0));
        // display the framtime in the top left
        ctx.print(0, 0, ctx.frame_time_ms);

        // take the player input and move the user
        player_input(self, ctx);

        self.run_systems();

        ctx.print(1, 1, "Hello Bracket World");

        let int_positions = self.ecs.read_storage::<IntPosition>();
        let float_positions = self.ecs.read_storage::<FloatPosition>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&int_positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }


        for (pos, render) in (&float_positions, &renderables).join() {
            ctx.set(pos.x as i32, pos.y as i32, render.fg, render.bg, render.glyph); // TODO!() always rounds down 
        }

        let mut last = self.ecs.write_resource::<LastTickInstant>();
        *last = LastTickInstant(web_time::Instant::now());
        // set for next tick
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(160,90)?
        .with_title("Flappy birb Terminal")
        .with_fps_cap(144.0)
        .with_fullscreen(false)
        .with_fitscreen(true)
        .build()?;

    let mut gs = State::new();
    // register the components
    gs.ecs.register::<IntPosition>();
    gs.ecs.register::<FloatPosition>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Velocity>();
    gs.ecs.register::<Player>();

    // set the last tick instant to now
    gs.ecs.insert::<LastTickInstant>(LastTickInstant(web_time::Instant::now()));

    // Create Player
    gs.ecs.create_entity()
    .with(FloatPosition {x: 40.0, y: 25.0})
    .with(Renderable {
        glyph: to_cp437('P'),
        bg: RGB::from_u8(0,0,0),
        fg: RGB::from_u8(255, 255, 0),
    })
    .with(Player{})
    .build();


    // Create Static Object
    gs.ecs.create_entity()
    .with(IntPosition {x: 20, y: 25})
    .with(Renderable {
        glyph: to_cp437('#'),
        bg: RGB::from_u8(0,0,0),
        fg: RGB::from_u8(64, 64, 64),
    })
    .build();

    // create moving tubes
    for i in 0..10 {
        gs.ecs.create_entity()
        .with(FloatPosition { x: i as f32 * 6.0 + 20.0 , y: 20.0 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::from_u8(0, 255, 0),
            bg: RGB::new(),
        })
        .with(Velocity::from_i32(2, 1))
        .build();
    }


    main_loop(context, gs)
}
