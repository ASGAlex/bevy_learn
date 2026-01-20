use bevy::app::{Plugin, Update};

use crate::tile_destructor::destructor::destructor_remove_tiles;

pub mod destructor;

#[derive(Default)]
pub struct TileDestructorPlugin;

impl Plugin for TileDestructorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, destructor_remove_tiles);
    }
}
