use bevy::prelude::*;

use crate::{
    game::map_tiles::{brick::Brick, tree::Tree, water::Water},
    utils::tiled::map_tile_type::{MapTileTypePlugin, MapTileTypeRegistry},
};

pub mod brick;
pub mod tree;
pub mod water;

pub struct MapTilesPlugin;

impl Plugin for MapTilesPlugin {
    fn build(&self, app: &mut App) {
        let mut map_tile_type_registry = MapTileTypeRegistry::default();
        map_tile_type_registry
            .register::<Tree>()
            .register::<Brick>()
            .register::<Water>();

        app.insert_resource(map_tile_type_registry);
        app.add_plugins(MapTileTypePlugin);
    }
}
