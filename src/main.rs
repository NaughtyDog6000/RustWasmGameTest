use web_time::{Instant, SystemTime};
use bracket_terminal::prelude::*;

struct Birb {
    x: i32,
    y: i32,
    // rotation: i32,
}

struct Logic {
    // the time that the last logic tick happened
    next_logic_tick: u128,
    start_instant: Instant,
    // the time between logic ticks in ms.
    logic_tick_time: u128,
}
struct State {
    player: Birb,
    game_logic: Logic, 
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear the screen and set background
        ctx.cls_bg(RGB::from_f32(0.5, 0.5, 1.0));
        // display the framtime in the top left
        ctx.print(0, 0, ctx.frame_time_ms);

        let now = self.game_logic.start_instant.elapsed().as_millis();
        // if enough time has passed since the last logic tick then run the logic, otherwise re-render the current frame.
        if now > self.game_logic.next_logic_tick {
            self.game_logic.next_logic_tick = now + self.game_logic.logic_tick_time;
            self.player.x += 1;

        }

        ctx.print(1, 1, "Hello Bracket World");
        ctx.print_color(self.player.x, self.player.y, RGB::named(WHITE), RGB::named(RED), "P");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple(160,90)?
        .with_title("Flappy birb Terminal")
        .with_fps_cap(144.0)
        .with_fullscreen(false)
        .build()?;

    let gs: State = State { player: Birb { x: 0, y: 10 }, game_logic: Logic {next_logic_tick: 0, logic_tick_time: 500, start_instant: web_time::Instant::now() } };
    main_loop(context, gs)
}