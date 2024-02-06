
struct Birb {
    x: i32,
    y: i32,

}

impl Tick for Birb {
    fn tick(&mut self, ctx: &mut BTerm) {
        println!("hello");
    }
} 

impl Render for Birb {
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.print_color(self.x, self.y)
    }
}