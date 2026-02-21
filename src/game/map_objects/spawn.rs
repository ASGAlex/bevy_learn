use avian2d::prelude::{CollisionLayers, RigidBody, Sensor};
use bevy::{
    asset::{self, Assets},
    ecs::{
        component::Component,
        entity::Entity,
        observer::On,
        query::With,
        system::{Commands, EntityCommands, Query, Res},
    },
    reflect::Reflect,
    transform::components::Transform,
};
use bevy_ecs_tiled::prelude::{
    ObjectCreated, TileCreated, TiledEvent, TiledMapAsset, TiledObject,
    tiled::{Layer, Object},
};

use crate::{
    game::GameLayer,
    utils::tiled::{map_object_type::MapObjectType, map_tile_type::MapItemBasic},
};

#[derive(Component, Reflect)]
pub struct Spawn;

#[derive(Component)]
pub struct PlayerSpawn;

impl MapItemBasic for Spawn {
    fn layer_class() -> String {
        "spawn".to_string()
    }

    fn class() -> String {
        "spawn".to_string()
    }

    fn on_collision_created(
        entity_commands: &mut EntityCommands,
        _assets: &Res<Assets<TiledMapAsset>>,
        _data: &Layer,
    ) {
        entity_commands
            .insert(RigidBody::Static)
            .insert(Sensor)
            .insert(Spawn)
            .insert(CollisionLayers::new(GameLayer::Trees, [GameLayer::Player]));
    }
}

impl MapObjectType for Spawn {
    fn on_object_created(
        commands: &mut Commands,
        _assets: &Res<Assets<TiledMapAsset>>,
        object_created: &On<TiledEvent<ObjectCreated>>,
        object: &Object,
    ) {
        if object.user_type == "spawn_player" {
            let object_entity = object_created.event().origin;
            commands.entity(object_entity).insert(PlayerSpawn);
        }
    }
}
