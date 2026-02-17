use std::collections::HashMap;

use avian2d::prelude::{PhysicsLayer, RigidBody};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{
    ColliderCreated, TileCreated, TiledColliderSource, TiledEvent, TiledMapAsset, tiled::Layer,
};

use crate::game::map_objects::{brick::Brick, tree::Tree, water::Water};

pub mod brick;
pub mod spawn;
pub mod tree;
pub mod water;

pub struct MapObjectsPlugin;

impl Plugin for MapObjectsPlugin {
    fn build(&self, app: &mut App) {
        let mut map_objects_registry = MapObjectsRegistry::default();
        map_objects_registry
            .register::<Tree>()
            .register::<Brick>()
            .register::<Water>();

        app.add_observer(on_collider_created)
            .add_observer(on_tile_created)
            .insert_resource(map_objects_registry);
    }
}

type OnCollisionCreatedFn = fn(&mut EntityCommands, &Res<Assets<TiledMapAsset>>, &Layer);
type OnTileCreatedFn = fn(&mut Commands, &Res<Assets<TiledMapAsset>>, &On<TiledEvent<TileCreated>>);

#[derive(Default, Resource)]
struct MapObjectsRegistry {
    on_collision_created: HashMap<String, OnCollisionCreatedFn>,
    on_tile_created: HashMap<String, OnTileCreatedFn>,
}

impl MapObjectsRegistry {
    pub fn register<T: MapObject>(&mut self) -> &mut MapObjectsRegistry {
        let name = T::name();
        let layer_name = T::layer_name();

        self.on_collision_created
            .insert(layer_name, T::on_collision_created);

        self.on_tile_created.insert(name, T::on_tile_created);
        self
    }
}

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Ground,
    Water,
    Player,
    Bricks,
    Trees,
    Sky,
}

pub trait MapObject {
    fn layer_name() -> String;
    fn name() -> String;

    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        assets: &Res<Assets<TiledMapAsset>>,
        data: &Layer,
    );

    fn on_tile_created(
        commands: &mut Commands,
        assets: &Res<Assets<TiledMapAsset>>,
        tile_created: &On<TiledEvent<TileCreated>>,
    );
}

fn on_collider_created(
    collider_created: On<TiledEvent<ColliderCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    map_objects_registry: Res<MapObjectsRegistry>,
) {
    if collider_created.event().event.source == TiledColliderSource::TilesLayer {
        let mut entity_commands = commands.entity(collider_created.event().origin);
        entity_commands.insert(RigidBody::Static);
        match collider_created.event().get_layer(&assets) {
            None => {}
            Some(data) => {
                if let Some(tile_type) = &data.user_type
                    && let Some(function) = map_objects_registry.on_collision_created.get(tile_type)
                {
                    function(&mut entity_commands, &assets, &data);
                }
            }
        };
    }
}
fn on_tile_created(
    tile_created: On<TiledEvent<TileCreated>>,
    assets: Res<Assets<TiledMapAsset>>,
    mut commands: Commands,
    map_objects_registry: Res<MapObjectsRegistry>,
) {
    let tile = tile_created.event().get_tile(&assets);
    match tile {
        None => {}
        Some(tile) => match &tile.user_type {
            None => {}
            Some(tile_type) => {
                if let Some(function) = map_objects_registry.on_tile_created.get(tile_type) {
                    function(&mut commands, &assets, &tile_created);
                }
            }
        },
    }
}
