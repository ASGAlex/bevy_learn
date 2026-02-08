use crate::game::actors::movement::{LastMoveDir, PlayerLookDir, PlayerMoving};
use crate::game::actors::player::Player;
use crate::{CAMERA_DECAY_RATE, MainCamera};
use bevy::camera::{Camera2d, OrthographicProjection, Projection};
use bevy::ecs::component::Component;
use bevy::ecs::query::Without;
use bevy::ecs::system::{Query, ResMut};
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::FloatExt;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Res, Single, StableInterpolate, Time, Transform, With};

const LOOK_FORWARD_DISTANCE: f32 = 48.0;
const ZOOM_SPEED: f32 = 2.0;
const BASE_ZOOM: f32 = 0.4;

pub fn update_camera_position(
    mut camera: Single<(&mut Transform, &mut Projection), (With<Camera2d>, With<MainCamera>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    look_dir: Res<PlayerLookDir>,
    player_moving: Res<PlayerMoving>,
) {
    let idle_zoom = BASE_ZOOM * 1.1; // +10%

    // ===== ZOOM =====
    let target_zoom = if player_moving.is_moving {
        BASE_ZOOM
    } else {
        idle_zoom
    };

    let mut projection = &mut *camera.1;
    if let Projection::Orthographic(ortho) = &mut projection {
        ortho.scale = ortho
            .scale
            .lerp(target_zoom, 1.0 - (-ZOOM_SPEED * time.delta_secs()).exp());
    }

    let target_point = match look_dir.0 {
        Some(look_direction) => look_direction.to_vec3().mul_add(
            Vec3::new(LOOK_FORWARD_DISTANCE, LOOK_FORWARD_DISTANCE, 0.0),
            player.translation,
        ),
        None => player.translation,
    };
    let direction = Vec3::new(target_point.x, target_point.y, camera.0.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .0
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
