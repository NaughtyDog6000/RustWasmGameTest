use bracket_terminal::prelude::*;

struct Birb {
    x: i32,
    y: i32,
    rotation: i32,
}

struct State {
    player: Birb,

}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear the screen and set background
        ctx.cls_bg(RGB::from_f32(0.5, 0.5, 1.0));
        // display the framtime in the top left
        ctx.print(0, 0, ctx.frame_time_ms);

        ctx.print(1, 1, "Hello Bracket World");
        ctx.print_color(self.player.x, 4, RGB::named(WHITE), RGB::named(RED), "dafug");
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy birb Terminal")
        .build()?;

    let gs: State = State { player: Birb { x: 0, y: 0, rotation: 0 }  };
    main_loop(context, gs)
}