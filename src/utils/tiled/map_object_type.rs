use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{tiled::Object, *};

use crate::utils::tiled::map_tile_type::{MapItemBasic, OnCollisionCreatedFn};

pub struct MapObjectTypePlugin;

impl Plugin for MapObjectTypePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_collider_created)
            .add_observer(on_object_created)
            .init_resource::<MapObjectsTypeRegistry>();
    }
}
type OnObjectCreatedFn =
    fn(&mut Commands, &Res<Assets<TiledMapAsset>>, &On<TiledEvent<ObjectCreated>>, &Object);

#[derive(Default, Resource)]
pub struct MapObjectsTypeRegistry {
    on_collision_created: HashMap<String, OnCollisionCreatedFn>,
    on_object_created: HashMap<String, OnObjectCreatedFn>,
}
impl MapObjectsTypeRegistry {
    pub fn register<T: MapObjectType>(&mut self) -> &mut MapObjectsTypeRegistry {
        let class = T::class();
        let layer_class = T::layer_class();

        self.on_collision_created
            .insert(layer_class.clone(), T::on_collision_created);

        self.on_object_created
            .insert(layer_class.clone(), T::on_object_created);
        self
    }
}
pub trait MapObjectType: MapItemBasic {
    fn on_object_created(
        commands: &mut Commands,
        assets: &Res<Assets<TiledMapAsset>>,
        tile_created_event: &On<TiledEvent<ObjectCreated>>,
        object_data: &Object,
    );
}

fn on_collider_created(
    collider_created_event: On<TiledEvent<ColliderCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    map_tile_type_registry: Res<MapObjectsTypeRegistry>,
) {
    if collider_created_event.event().event.source == TiledColliderSource::TilesLayer {
        let Some(data) = collider_created_event.event().get_layer(&assets) else {
            return;
        };

        let Some(layer_class) = &data.user_type else {
            return;
        };
        let Some(function) = map_tile_type_registry.on_collision_created.get(layer_class) else {
            return;
        };

        let mut entity_commands = commands.entity(collider_created_event.event().origin);
        function(&mut entity_commands, &assets, &data);
    }
}

fn on_object_created(
    object_created_event: On<TiledEvent<ObjectCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    map_objects_registry: Res<MapObjectsTypeRegistry>,
) {
    let Some(layer) = object_created_event.event().get_layer(&assets) else {
        return;
    };
    let Some(layer_class) = &layer.user_type else {
        return;
    };
    let Some(function) = map_objects_registry.on_object_created.get(layer_class) else {
        return;
    };

    let Some(object_data) = object_created_event.event().get_object(&assets) else {
        return;
    };
    function(&mut commands, &assets, &object_created_event, &object_data);
}
