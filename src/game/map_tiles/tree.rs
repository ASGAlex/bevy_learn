use avian2d::prelude::{CollisionLayers, RigidBody, Sensor};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{
    ObjectCreated, TiledEvent, TiledMapAsset, TiledObject,
    tiled::{Layer, Object},
};

use crate::{
    game::GameLayer,
    utils::tiled::map_tile_type::{MapItemBasic, MapTileType},
};

#[derive(Component, Reflect)]
pub struct Tree;

impl MapItemBasic for Tree {
    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        assets: &Res<Assets<TiledMapAsset>>,
        data: &Layer,
    ) {
        entity_commands
            .insert(RigidBody::Static)
            .insert(Sensor)
            .insert(Tree)
            .insert(CollisionLayers::new(GameLayer::Trees, [GameLayer::Player]));
    }

    fn class() -> String {
        "tree".to_string()
    }

    fn layer_class() -> String {
        "trees".to_string()
    }
}

impl MapTileType for Tree {
    fn on_tile_created(
        commands: &mut Commands,
        assets: &Res<Assets<TiledMapAsset>>,
        tile_created: &On<TiledEvent<bevy_ecs_tiled::prelude::TileCreated>>,
    ) {
    }
}
