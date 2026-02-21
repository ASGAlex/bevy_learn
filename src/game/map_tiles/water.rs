use avian2d::prelude::{CollisionLayers, RigidBody};
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
pub struct Water;

impl MapItemBasic for Water {
    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        assets: &Res<Assets<TiledMapAsset>>,
        data: &Layer,
    ) {
        entity_commands
            .insert(RigidBody::Static)
            .insert(Water)
            .insert(CollisionLayers::new(GameLayer::Water, [GameLayer::Player]));
    }

    fn class() -> String {
        "water".to_string()
    }

    fn layer_class() -> String {
        "water".to_string()
    }
}

impl MapTileType for Water {
    fn on_tile_created(
        commands: &mut Commands,
        assets: &Res<Assets<TiledMapAsset>>,
        tile_created: &On<TiledEvent<bevy_ecs_tiled::prelude::TileCreated>>,
    ) {
    }
}
