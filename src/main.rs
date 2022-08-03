#[forbid(warnings)]

use rltk::{FontCharType, GameState, Rltk, RltkBuilder, RGB};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: FontCharType,
    foreground: RGB,
    background: RGB,
}

struct State {
    world: World,
}

impl GameState for State {

    fn tick(&mut self, context: &mut Rltk) {
        context.cls();

        let positions = self.world.read_storage::<Position>();
        let renderables = self.world.read_storage::<Renderable>();

        for (position, renderable) in (&positions, &renderables).join() {
            context.set(position.x, position.y, renderable.foreground, renderable.background, renderable.glyph);
        }
    }
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut state = State {
        world: World::new(),
    };

    state.world.register::<Position>();
    state.world.register::<Renderable>();

    state
        .world
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            foreground: RGB::named(rltk::YELLOW),
            background: RGB::named(rltk::BLACK),
        })
        .build();

    rltk::main_loop(context, state)
}
