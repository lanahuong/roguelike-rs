use bracket_lib::prelude::*;
use specs::{prelude::*, World};
use specs_derive::Component;
use std::cmp::{max, min};

/// The game state
struct State {
    /// The ECS system
    ecs: World,
}

/// A component for the position
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

/// A component for rendering
#[derive(Component)]
struct Renderable {
    /// The glyph (or character) to render
    glyph: FontCharType,
    /// Foreground color to render
    fg: RGB,
    /// Background color to render
    bg: RGB,
}

/// A component for entities that move to the left
#[derive(Component)]
struct LeftMover {}

/// A component for the entity controlled by the player
#[derive(Component)]
struct Player {}

/// A system that make entities walk to the left
struct LeftWalker {}

impl<'a> System<'a> for LeftWalker {
    type SystemData = (ReadStorage<'a, LeftMover>, WriteStorage<'a, Position>);

    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_, pos) in (&lefty, &mut pos).join() {
            pos.x -= 1;
            if pos.x < 0 {
                pos.x = 79;
            }
        }
    }
}

impl State {
    /// Runs the systems in the game state
    fn run_systems(&mut self) {
        let mut lw = LeftWalker {};
        lw.run_now(&self.ecs);
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

        // Display entities with a position that can be rendered
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

/// Move the player inside the console boundaries
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for (_, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

/// Apply effects from player inputs
fn player_input(gs: &mut State, ctx: &mut BTerm) {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            _ => {}
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

    // Create monsters entities that move left
    for i in 0..10 {
        gs.ecs
            .create_entity()
            .with(Position { x: i * 7, y: 20 })
            .with(Renderable {
                glyph: to_cp437('☺'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .with(LeftMover {})
            .build();
    }

    // Call the game loop which calls the state handler every tick
    main_loop(context, gs)
}
