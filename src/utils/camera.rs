use crate::game::actors::movement::{LastMoveDir, PlayerLookDir, PlayerMoving};
use crate::game::actors::player::Player;
use crate::{CAMERA_DECAY_RATE, MainCamera};
use bevy::app::{Plugin, PostUpdate};
use bevy::camera::{Camera2d, OrthographicProjection, Projection};
use bevy::ecs::component::Component;
use bevy::ecs::query::Without;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::{Query, ResMut};
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::FloatExt;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Res, Single, StableInterpolate, Time, Transform, With};
use bevy::time::{Timer, TimerMode};

const LOOK_FORWARD_DISTANCE: f32 = 48.0;
const ZOOM_SPEED: f32 = 2.0;
const BASE_ZOOM: f32 = 0.3;

#[derive(Default)]
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(PostUpdate, update_camera_position);
        app.insert_resource(CameraZoomState {
            mode: ZoomMode::Moving,
            switch_timer: Timer::from_seconds(2.0, TimerMode::Once), // задержка 2 сек
        });
    }
}

#[derive(Resource)]
pub struct CameraZoomState {
    pub mode: ZoomMode,
    pub switch_timer: Timer,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ZoomMode {
    Moving,
    Idle,
}

pub fn update_camera_position(
    mut camera: Single<(&mut Transform, &mut Projection), (With<Camera2d>, With<MainCamera>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    look_dir: Res<PlayerLookDir>,
    player_moving: Res<PlayerMoving>,
    mut zoom_state: ResMut<CameraZoomState>,
) {
    let idle_zoom = BASE_ZOOM * 1.1;

    // ===== MODE SWITCH WITH DELAY =====
    let desired_mode = if player_moving.is_moving {
        ZoomMode::Moving
    } else {
        ZoomMode::Idle
    };

    if desired_mode != zoom_state.mode {
        zoom_state.switch_timer.tick(time.delta());

        if zoom_state.switch_timer.is_finished() {
            zoom_state.mode = desired_mode;
            zoom_state.switch_timer.reset();
        }
    } else {
        zoom_state.switch_timer.reset();
    }

    // ===== ZOOM =====
    let target_zoom = match zoom_state.mode {
        ZoomMode::Moving => BASE_ZOOM,
        ZoomMode::Idle => idle_zoom,
    };

    if let Projection::Orthographic(ortho) = &mut *camera.1 {
        ortho.scale = ortho
            .scale
            .lerp(target_zoom, 1.0 - (-ZOOM_SPEED * time.delta_secs()).exp());
    }

    // ===== POSITION =====
    let target_point = match look_dir.0 {
        Some(look_direction) => look_direction.to_vec3().mul_add(
            Vec3::new(LOOK_FORWARD_DISTANCE, LOOK_FORWARD_DISTANCE, 0.0),
            player.translation,
        ),
        None => player.translation,
    };

    let direction = Vec3::new(target_point.x, target_point.y, camera.0.translation.z);

    camera
        .0
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}
