mod game;
mod utils;

use crate::game::actors::movement::*;
use crate::game::actors::player::*;
use crate::game::weapons::bullet::*;
use crate::utils::camera::*;
use crate::utils::destructor::*;
use crate::utils::pool::*;
use crate::utils::region_deactivation::RegionActivationPlugin;
use crate::utils::region_deactivation::RegionAware;
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::tiled::PropertyValue;
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;
const MAP_CHUNK_SIZE: f32 = 400.0;
const PHYSICS_SPEED: f32 = 0.3;
/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 1.;

/// Player movement speed factor.
const PLAYER_SPEED: f32 = 50. / PHYSICS_SPEED;

#[derive(Component)]
struct MainCamera;

fn main() {
    App::new()
        // Bevy default plugins: prevent blur effect by changing default sampling
        .add_plugins(DefaultPlugins.build().set(ImagePlugin::default_nearest()))
        .add_plugins(TiledPlugin::default())
        .add_plugins((
            PhysicsPlugins::default()
                .with_length_unit(1.)
                .set(PhysicsInterpolationPlugin::interpolate_all()),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            PhysicsDebugPlugin,
            // TiledDebugTilesPlugin::default(),
        ))
        .insert_resource(Time::<Physics>::default().with_relative_speed(PHYSICS_SPEED))
        .add_plugins(EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(SpritesheetAnimationPlugin)
        .add_plugins(TileDestructorPlugin)
        .add_plugins(ShootingPlugin)
        .add_plugins(RegionActivationPlugin)
        .add_plugins(PoolPlugin::<Bullet>::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(LastMoveDir::default())
        .init_resource::<PlayerLookDir>()
        .init_resource::<PlayerMoving>()
        .add_systems(Startup, (init).chain())
        .add_systems(
            FixedUpdate,
            (
                spawn_player,
                zoom,
                move_player,
                // interpolate_player_position,
                apply_player_look_dir.after(move_player),
                player_animation_controller.after(move_player),
            ),
        )
        .add_systems(PostUpdate, update_camera)
        .run();
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut projection = OrthographicProjection::default_2d();
    projection.scale = 0.3;
    commands.spawn((Camera2d, Projection::Orthographic(projection), MainCamera));

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
            TiledWorldChunking::new(MAP_CHUNK_SIZE, MAP_CHUNK_SIZE),
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
