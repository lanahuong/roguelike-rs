use super::{Map, Player, Position, Viewshed};
use bracket_lib::prelude::*;
use specs::prelude::*;

/// System to check handle visibility of entities
pub struct VisibilitySytem {}

impl<'a> System<'a> for VisibilitySytem {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;
        // Update the visible tiles of all entities who can see
        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
                // Only keep the tiles inside the map boundaries (aka valid tiles)
                viewshed.visible_tiles.retain(|p| {
                    p.x >= 0 && p.x < map.width as i32 && p.y >= 0 && p.y < map.height as i32
                });

                // If the entity is a player change the visible and revealed tiles of the map with the viewshed
                let p: Option<&Player> = player.get(ent);
                if let Some(_p) = p {
                    for t in map.visible_tiles.iter_mut() {
                        *t = false
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = map.xy_idx(vis.x, vis.y);
                        map.visible_tiles[idx] = true;
                        map.revealed_tiles[idx] = true;
                    }
                }
            }
        }
    }
}
