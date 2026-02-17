use avian2d::prelude::CollisionLayers;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TiledMapAsset, tiled::Layer};

use crate::game::map_objects::{GameLayer, MapObject};

#[derive(Component, Reflect)]
pub struct Water;

impl MapObject for Water {
    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        assets: &Res<Assets<TiledMapAsset>>,
        data: &Layer,
    ) {
        entity_commands
            .insert(Water)
            .insert(CollisionLayers::new(GameLayer::Water, [GameLayer::Player]));
    }

    fn name() -> String {
        "water".to_string()
    }

    fn layer_name() -> String {
        "water".to_string()
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
