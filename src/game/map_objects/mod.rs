use bevy::prelude::*;

use crate::{
    game::map_objects::spawn::Spawn,
    utils::tiled::map_object_type::{MapObjectTypePlugin, MapObjectsTypeRegistry},
};

pub mod spawn;

pub struct MapObjectsPlugin;

impl Plugin for MapObjectsPlugin {
    fn build(&self, app: &mut App) {
        let mut map_objects_type_registry = MapObjectsTypeRegistry::default();
        map_objects_type_registry.register::<Spawn>();

        app.insert_resource(map_objects_type_registry);
        app.add_plugins(MapObjectTypePlugin);
    }
}
