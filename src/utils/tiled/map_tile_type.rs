use std::collections::HashMap;

use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{
    ColliderCreated, TileCreated, TiledColliderSource, TiledEvent, TiledMapAsset, tiled::Layer,
};

pub struct MapTileTypePlugin;

impl Plugin for MapTileTypePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_collider_created)
            .add_observer(on_tile_created)
            .init_resource::<MapTileTypeRegistry>();
    }
}

pub(crate) type OnCollisionCreatedFn = fn(&mut EntityCommands, &Res<Assets<TiledMapAsset>>, &Layer);
type OnTileCreatedFn = fn(&mut Commands, &Res<Assets<TiledMapAsset>>, &On<TiledEvent<TileCreated>>);

#[derive(Default, Resource)]
pub struct MapTileTypeRegistry {
    on_collision_created: HashMap<String, OnCollisionCreatedFn>,
    on_tile_created: HashMap<String, OnTileCreatedFn>,
}

impl MapTileTypeRegistry {
    pub fn register<T: MapTileType>(&mut self) -> &mut MapTileTypeRegistry {
        let class = T::class();
        let layer_class = T::layer_class();

        self.on_collision_created
            .insert(layer_class.clone(), T::on_collision_created);

        self.on_tile_created
            .insert(class.clone(), T::on_tile_created);

        self
    }
}

pub trait MapItemBasic {
    fn layer_class() -> String;
    fn class() -> String;

    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        assets: &Res<Assets<TiledMapAsset>>,
        data: &Layer,
    );
}

pub trait MapTileType: MapItemBasic {
    fn on_tile_created(
        commands: &mut Commands,
        assets: &Res<Assets<TiledMapAsset>>,
        tile_created: &On<TiledEvent<TileCreated>>,
    );
}

fn on_collider_created(
    collider_created_event: On<TiledEvent<ColliderCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    map_tile_type_registry: Res<MapTileTypeRegistry>,
) {
    if collider_created_event.event().event.source == TiledColliderSource::TilesLayer {
        let mut entity_commands = commands.entity(collider_created_event.event().origin);
        let Some(data) = collider_created_event.event().get_layer(&assets) else {
            return;
        };

        let Some(layer_class) = &data.user_type else {
            return;
        };
        let Some(function) = map_tile_type_registry.on_collision_created.get(layer_class) else {
            return;
        };

        function(&mut entity_commands, &assets, &data);
    }
}
fn on_tile_created(
    tile_created_event: On<TiledEvent<TileCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    map_objects_registry: Res<MapTileTypeRegistry>,
) {
    let Some(tile) = tile_created_event.event().get_tile(&assets) else {
        return;
    };

    let Some(tile_class) = &tile.user_type else {
        return;
    };

    let Some(function) = map_objects_registry.on_tile_created.get(tile_class) else {
        return;
    };

    function(&mut commands, &assets, &tile_created_event);
}
