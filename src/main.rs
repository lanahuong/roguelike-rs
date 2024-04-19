use bracket_lib::prelude::*;
use specs::{prelude::*, World};

mod components;
use components::*;
mod map;
use map::*;
mod player;
use player::*;

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

    // Store a map
    gs.ecs.insert(new_map());

    // Create the player entity
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .with(Player {})
        .build();

    // Call the game loop which calls the state handler every tick
    main_loop(context, gs)
}
