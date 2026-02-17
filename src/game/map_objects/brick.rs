use avian2d::prelude::CollisionLayers;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMapAsset, tiled::Layer};

use crate::{
    game::map_objects::{GameLayer, MapObject},
    utils::destructor::AffectedByDestructor,
};

#[derive(Component, Reflect)]
pub struct Brick;

impl MapObject for Brick {
    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        assets: &Res<Assets<TiledMapAsset>>,
        data: &Layer,
    ) {
        entity_commands
            .insert(Brick)
            .insert(CollisionLayers::new(GameLayer::Bricks, [GameLayer::Player]));
    }

    fn name() -> String {
        "brick".to_string()
    }

    fn layer_name() -> String {
        "bricks".to_string()
    }

    fn on_tile_created(
        commands: &mut Commands,
        assets: &Res<Assets<TiledMapAsset>>,
        tile_created: &On<
            bevy_ecs_tiled::prelude::TiledEvent<bevy_ecs_tiled::prelude::TileCreated>,
        >,
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
