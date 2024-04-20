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
struct State {
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

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

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
    let (rooms, map) = new_map_rooms_and_corridors(30);
    // Store a map
    gs.ecs.insert(map);
    // Get the initial position of the player in the center of the first room
    let player_pos = rooms[0].center();

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
