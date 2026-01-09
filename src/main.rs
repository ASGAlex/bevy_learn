mod controls;
mod objects;
mod player;

use crate::controls::{move_player, update_camera, zoom};
use crate::objects::{GameObject, ObjectsCounter};
use crate::player::{Player, spawn_player};
use avian2d::prelude::*;
use bevy::ecs::query::QueryEntityError;
use bevy::ecs::relationship::Relationship;
use bevy::ecs::system::SystemState;
use bevy::ecs::world::FilteredEntityMut;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::tiled::{PropertyValue, Tile};
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use std::ops::Deref;
use bevy::platform::collections::Equivalent;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 2.;

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 100.;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        // Bevy default plugins: prevent blur effect by changing default sampling
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins(TiledPlugin::default())
        .add_plugins((
            PhysicsPlugins::default().with_length_unit(1.),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            // PhysicsDebugPlugin,
            // TiledDebugTilesPlugin::default(),
        ))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(Gravity(Vec2::default()))
        .insert_resource(ObjectsCounter::new())
        .add_systems(Startup, (init, spawn_player).chain())
        .add_systems(
            Update,
            (zoom, update_camera, move_player, colliding_with_player),
        )
        .run();
}

fn init(
    mut commands: Commands,
    mut counter: ResMut<ObjectsCounter>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((Camera2d, MainCamera));

    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));

    commands.spawn((
        Text::new("Move the light with WASD.\nThe camera will smoothly track the light."),
        Node {
            position_type: PositionType::Absolute,
            bottom: px(12),
            left: px(12),
            ..default()
        },
    ));

    // Load and spawn the world
    commands
        .spawn((
            TiledWorld(asset_server.load("tiles/learn.world")),
            TiledWorldChunking::new(50., 50.),
            TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
                ..default()
            },
        ))
        .observe(
            |collider_created: On<TiledEvent<ColliderCreated>>,
             assets: Res<Assets<TiledMapAsset>>,
             mut commands: Commands| {
                if collider_created.event().event.source == TiledColliderSource::TilesLayer {
                    let mut entity_commands = commands.entity(collider_created.event().origin);
                    entity_commands.insert(RigidBody::Static);
                    match collider_created.event().get_layer(&assets) {
                        None => {}
                        Some(data) => {
                            if data.name == "trees" {
                                entity_commands.insert(Sensor).insert(Tree).insert(
                                    CollisionLayers::new(GameLayer::Trees, [GameLayer::Player]),
                                );
                            } else if data.name == "water" {
                                entity_commands.insert(Water).insert(CollisionLayers::new(
                                    GameLayer::Water,
                                    [GameLayer::Player],
                                ));
                            } else if data.name == "bricks" {
                                entity_commands.insert(Brick).insert(CollisionLayers::new(
                                    GameLayer::Bricks,
                                    [GameLayer::Player],
                                ));
                            }
                        }
                    };
                }
            },
        )
        .observe(
            |tile_created: On<TiledEvent<TileCreated>>,
             assets: Res<Assets<TiledMapAsset>>,
             mut commands: Commands| {
                // dbg!(tile_created.event().get_tile_entity());
                // dbg!(tile_created.event().get_tile(&assets).unwrap());

                match tile_created.event().get_tile(&assets) {
                    None => {}
                    Some(tile) => match tile.properties.get("name") {
                        None => {}
                        Some(name) => {
                            if let PropertyValue::StringValue(data) = name {
                                if data == "brick" {
                                    match tile_created.event().get_tile_entity() {
                                        None => {}
                                        Some(entity) => {
                                            commands.entity(entity).insert(Brick);
                                        }
                                    }
                                }
                            }
                        }
                    },
                }
            },
        );
}

#[derive(Component)]
struct Tree;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Water;

fn colliding_with_player(
    mut commands: Commands,
    player: Single<(Entity, &Transform), With<Player>>,
    collisions: Collisions,
    brick_tiles: Query<(Entity, &TilePos), With<Brick>>,
    tilemap_q: Query<(
        &Name,
        &TilemapSize,
        &TilemapGridSize,
        &TilemapTileSize,
        &TilemapType,
        &TileStorage,
        &Transform,
        &TilemapAnchor,
    )>,
) {
    let (player_entity, player_transform) = player.into_inner();

    for contacts in collisions.collisions_with(player_entity) {

        let body = match (contacts.body1, contacts.body2) {
            (Some(b), _) if b != player_entity => b,
            (_, Some(b)) if b != player_entity => b,
            _ => continue,
        };

        let point = if let Some(contact) = contacts.find_deepest_contact() {
            contact.point.clone()
        } else {
            continue;
        };

        for (
            name,
            map_size,
            grid_size,
            tile_size,
            map_type,
            tile_storage,
            map_transform,
            anchor,
        ) in tilemap_q.iter() {
            if name.as_str() != "TiledTilemap(bricks, bricks)" {
                // continue;
            }
            // let point_in_map_pos: Vec2 = {
            //     let point_pos = Vec4::from((point, 0.0, 1.0));
            //     let point_in_map_pos = map_transform.to_matrix().inverse() * point_pos;
            //     point_in_map_pos.xy()
            // };
            let offset = Vec2::new(4.,4.);//anchor.as_offset(map_size, grid_size, tile_size, map_type);
            let offset2 = anchor.as_offset(map_size, grid_size, tile_size, map_type);
            let pos = point - offset;
            // dbg!(&name, &offset, &offset2,&pos);
            let x = ((pos.x / grid_size.x) + 0.5).floor() as i32;
            let y = ((pos.y / grid_size.y) + 0.5).floor() as i32;


            let x = ((pos.x / grid_size.x) + 0.5).floor() as i32;
            let y = ((pos.y / grid_size.y) + 0.5).floor() as i32;

            if let Some(tile_pos) = TilePos::from_i32_pair(x, y, map_size) {
                if let Some(tile_entity) = tile_storage.get(&tile_pos) {
                    dbg!((x, y, tile_entity));
                    commands.entity(tile_entity).despawn();
                }
            }


            // if let Some(tile_pos) = TilePos::from_world_pos(
            //     &point,
            //     map_size,
            //     grid_size,
            //     tile_size,
            //     map_type,
            //     anchor,
            // ) {
            // }
        }
        // match brick_tiles.get(body) {
        //     Ok(tile) => {
        //         dbg!(tile);
        //     }
        //     Err(error) => {
        //         dbg!(error);
        //     }
        // }
    }
}

#[derive(PhysicsLayer, Default)]
enum GameLayer {
    #[default]
    Ground,
    Water,
    Player,
    Bricks,
    Trees,
    Sky,
}
