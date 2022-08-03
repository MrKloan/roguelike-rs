use rltk::{GameState, Rltk};

struct State {}

impl GameState for State {
    
    fn tick(&mut self, context: &mut Rltk) {
        context.cls();
        context.print(1, 1, "Hello World!");
    }
}

fn main() -> rltk::BError {
    let context = rltk::RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let state = State {};
    rltk::main_loop(context, state)
}
