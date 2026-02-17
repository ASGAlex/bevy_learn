use avian2d::prelude::{CollisionLayers, Sensor};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMapAsset, tiled::Layer};

use crate::game::map_objects::{GameLayer, MapObject};

#[derive(Component, Reflect)]
pub struct Tree;

impl MapObject for Tree {
    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        assets: &Res<Assets<TiledMapAsset>>,
        data: &Layer,
    ) {
        entity_commands
            .insert(Sensor)
            .insert(Tree)
            .insert(CollisionLayers::new(GameLayer::Trees, [GameLayer::Player]));
    }

    fn name() -> String {
        "tree".to_string()
    }

    fn layer_name() -> String {
        "trees".to_string()
    }

    fn on_tile_created(
        commands: &mut Commands,
        assets: &Res<Assets<TiledMapAsset>>,
        tile_created: &On<
            bevy_ecs_tiled::prelude::TiledEvent<bevy_ecs_tiled::prelude::TileCreated>,
        >,
    ) {
    }
}
