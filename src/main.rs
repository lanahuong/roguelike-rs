use bracket_lib::prelude::*;
use specs::{prelude::*, World};

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;

/// The game state
pub struct State {
    /// The ECS system
    ecs: World,
}

impl State {
    /// Runs the systems in the game state
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clears the console
        ctx.cls();

        // Evolve the game state
        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Map>();
        map.draw_map(ctx);

        // Display entities with a position that can be rendered
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() -> BError {
    // Creates a console
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tuto")
        .build()?;

    // Creates a game state
    let mut gs: State = State { ecs: World::new() };

    // Register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();

    // Create a random map
    let map = Map::new_map_rooms_and_corridors(30);
    // Get the initial position of the player in the center of the first room
    let player_pos = if map.rooms.is_empty() {(40,25)} else {map.rooms[0].center()};
    // Store a map
    gs.ecs.insert(map);

    // Create the player entity
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_pos.0,
            y: player_pos.1,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(GREEN),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .build();

    // Call the game loop which calls the state handler every tick
    main_loop(context, gs)
}
