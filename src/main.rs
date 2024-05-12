use bracket_lib::prelude::*;
use specs::{prelude::*, World};

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
mod visibility_system;
use visibility_system::VisibilitySytem;
mod monster_ai_system;
use monster_ai_system::MonsterAISystem;

/// A the state machine to handle turns
#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

/// The game state
pub struct State {
    /// The ECS system
    ecs: World,
    run_state: RunState,
}

impl State {
    /// Runs the systems in the game state
    fn run_systems(&mut self) {
        let mut vis = VisibilitySytem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAISystem {};
        mob.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Clears the console
        ctx.cls();

        // Evolve the game state
        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        // Display all entities that have a position and can be rendered
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

fn main() -> BError {
    // Creates a console
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tuto")
        .build()?;

    // Creates a game state
    let mut gs: State = State {
        ecs: World::new(),
        run_state: RunState::Running,
    };

    // Register components
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<LeftMover>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    // Create a random map
    let map = Map::new_map_rooms_and_corridors(30);
    // Get the initial position of the player in the center of the first room
    let player_pos = if map.rooms.is_empty() {
        (40, 25)
    } else {
        map.rooms[0].center()
    };

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
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    // Add monsters
    let mut rng = RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let (glyph, name) = match rng.roll_dice(1, 2) {
            1 => (to_cp437('g'), "Goblin".to_string()),
            _ => (to_cp437('o'), "Orc".to_string()),
        };
        gs.ecs
            .create_entity()
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
            .with(Name {
                name: format!("{} #{}", &name, i),
            })
            .build();
    }

    // Store game resources (map and player position)
    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_pos.0, player_pos.1));

    // Call the game loop which calls the state handler every tick
    main_loop(context, gs)
}
