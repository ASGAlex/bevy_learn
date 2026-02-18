mod game;
mod utils;

use crate::game::actors::movement::*;
use crate::game::actors::player::*;
use crate::game::map_objects::MapObjectsPlugin;
use crate::game::weapons::bullet::*;
use crate::utils::camera::*;
use crate::utils::destructor::*;
use crate::utils::region_deactivation::RegionActivationPlugin;
use avian2d::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_spritesheet_animation::plugin::SpritesheetAnimationPlugin;

const MAP_CHUNK_SIZE: f32 = 400.0;
const PHYSICS_SPEED: f32 = 0.3;
/// How quickly should the camera snap to the desired location.
const CAMERA_DECAY_RATE: f32 = 1.1;

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
        .add_plugins((
            TileDestructorPlugin,
            ShootingPlugin,
            RegionActivationPlugin,
            MapObjectsPlugin,
            GameCameraPlugin,
        ))
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(LogDiagnosticsPlugin::default())
        .insert_resource(Gravity(Vec2::ZERO))
        .insert_resource(LastMoveDir::default())
        .init_resource::<PlayerLookDir>()
        .init_resource::<PlayerMoving>()
        .add_systems(Startup, (init).chain())
        .add_systems(
            FixedUpdate,
            (
                spawn_player,
                move_player,
                apply_player_look_dir.after(move_player),
                player_animation_controller.after(move_player),
            ),
        )
        // .add_systems(PostUpdate, update_camera_position)
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
    commands.spawn((
        TiledWorld(asset_server.load("tiles/learn.world")),
        TiledWorldChunking::new(MAP_CHUNK_SIZE, MAP_CHUNK_SIZE),
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            backend: TiledPhysicsAvianBackend::Polyline,
            ..default()
        },
    ));
}
