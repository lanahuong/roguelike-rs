use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

/// A component for the position
#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

/// A component for rendering
#[derive(Component)]
pub struct Renderable {
    /// The glyph (or character) to render
    pub glyph: FontCharType,
    /// Foreground color to render
    pub fg: RGB,
    /// Background color to render
    pub bg: RGB,
}

/// A component for entities that move to the left
#[derive(Component)]
pub struct LeftMover {}

/// A component for the entity controlled by the player
#[derive(Component)]
pub struct Player {}

/// A component for characters with a field of view
#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

/// A component for monster entities
#[derive(Component)]
pub struct Monster {}

/// A component to name entities
#[derive(Component)]
pub struct Name {
    pub name: String,
}
