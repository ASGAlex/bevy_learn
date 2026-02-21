use avian2d::prelude::{CollisionLayers, RigidBody};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledEvent, TiledMapAsset, tiled::Layer};

use crate::{
    game::GameLayer,
    utils::tiled::{
        destructor::AffectedByDestructor,
        map_tile_type::{MapItemBasic, MapTileType},
    },
};

#[derive(Component, Reflect)]
pub struct Brick;

impl MapItemBasic for Brick {
    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        _assets: &Res<Assets<TiledMapAsset>>,
        _data: &Layer,
    ) {
        entity_commands
            .insert(RigidBody::Static)
            .insert(Brick)
            .insert(CollisionLayers::new(GameLayer::Bricks, [GameLayer::Player]));
    }

    fn class() -> String {
        "brick".to_string()
    }

    fn layer_class() -> String {
        "bricks".to_string()
    }
}

impl MapTileType for Brick {
    fn on_tile_created(
        commands: &mut Commands,
        _assets: &Res<Assets<TiledMapAsset>>,
        tile_created: &On<TiledEvent<bevy_ecs_tiled::prelude::TileCreated>>,
    ) {
        match tile_created.event().get_tile_entity() {
            None => {}
            Some(entity) => {
                commands.entity(entity).insert(Brick);
                if let Some(tilemap_entity) = tile_created.event().get_tilemap_entity() {
                    commands
                        .entity(tilemap_entity)
                        .insert(AffectedByDestructor {
                            layer_id: tile_created.get_layer_index().unwrap(),
                        });
                }
            }
        }
    }
}
