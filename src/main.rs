mod controls;
mod objects;
mod player;
mod shoot;
mod tile_destructor;

use crate::controls::{
    LastMoveDir, PlayerLookDir, PlayerMoving, apply_player_look_dir, interpolate_player_position,
    move_player, update_camera, zoom,
};
use crate::objects::{GameObject, ObjectsCounter};
use crate::player::{player_animation_controller, spawn_player};
use crate::shoot::ShootingPlugin;
use crate::tile_destructor::TileDestructorPlugin;
use crate::tile_destructor::destructor::AffectedByDestructor;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::tiled::PropertyValue;
use bevy_ecs_tiled::prelude::*;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;

/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 1.;

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 50.;

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
        // .add_plugins(EguiPlugin::default())
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(SpritesheetAnimationPlugin)
        .add_plugins(TileDestructorPlugin)
        .add_plugins(ShootingPlugin)
        .insert_resource(Gravity(Vec2::default()))
        .insert_resource(ObjectsCounter::new())
        .insert_resource(LastMoveDir::default())
        .init_resource::<PlayerLookDir>()
        .init_resource::<PlayerMoving>()
        .add_systems(Startup, (init).chain())
        .add_systems(Update, (spawn_player, zoom))
        .add_systems(
            FixedUpdate,
            (
                move_player,
                interpolate_player_position,
                apply_player_look_dir.after(move_player),
                player_animation_controller.after(move_player),
            ),
        )
        .add_systems(PostUpdate, update_camera)
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
            TiledWorldChunking::new(200., 200.),
            TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
                backend: TiledPhysicsAvianBackend::Polyline,
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
                            dbg!(&data.name);
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
                // dbg!(tile_created.event().g daet_tile_entity());
                // dbg!(tile_created.event().get_tile(&assets).unwrap());

                match tile_created.event().get_tile(&assets) {
                    None => {}
                    Some(tile) => match tile.properties.get("name") {
                        None => {}
                        Some(name) => match name {
                            PropertyValue::StringValue(data) if data == "brick" => {
                                match tile_created.event().get_tile_entity() {
                                    None => {}
                                    Some(entity) => {
                                        commands.entity(entity).insert(Brick);
                                        if let Some(tilemap_entity) =
                                            tile_created.event().get_tilemap_entity()
                                        {
                                            commands
                                                .entity(tilemap_entity)
                                                .insert(AffectedByDestructor);
                                        }
                                    }
                                }
                            }
                            _ => (),
                        },
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
