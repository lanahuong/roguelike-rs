use super::{Monster, Name, Viewshed};
use bracket_lib::prelude::*;
use bracket_lib::terminal::console;
use specs::prelude::*;

/// System to manage basic Monster AI
pub struct MonsterAISystem {}

impl<'a> System<'a> for MonsterAISystem {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, pos, monster, name) = data;
        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*pos) {
                console::log(&format!("{}: Shouts insults", name.name));
            }
        }
    }
}
