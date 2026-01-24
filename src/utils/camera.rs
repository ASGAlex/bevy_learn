use crate::game::actors::player::Player;
use crate::{CAMERA_DECAY_RATE, MainCamera};
use bevy::camera::{Camera2d, Projection};
use bevy::ecs::component::Component;
use bevy::ecs::system::Query;
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::Vec3;
use bevy::prelude::{Res, Single, StableInterpolate, Time, Transform, With};

pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, With<MainCamera>)>,
    // player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
    visual: Single<&VisualPosition, With<Player>>,
) {
    let Vec3 { x, y, .. } = visual.get();
    let direction = Vec3::new(x, y, camera.translation.z);

    // Applies a smooth effect to camera movement using stable interpolation
    // between the camera position and the player position on the x and y axes.
    camera
        .translation
        .smooth_nudge(&direction, CAMERA_DECAY_RATE, time.delta_secs());
}

pub fn zoom(
    projection: Single<&mut Projection, (With<Camera2d>, With<MainCamera>)>,
    mouse_wheel_input: Res<AccumulatedMouseScroll>,
) {
    if let Projection::Orthographic(orthographic) = projection.into_inner().as_mut() {
        // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
        let delta_zoom = -mouse_wheel_input.delta.y * 0.2;
        // When changing scales, logarithmic changes are more intuitive.
        // To get this effect, we add 1 to the delta, so that a delta of 0
        // results in no multiplicative effect, positive values result in a multiplicative increase,
        // and negative values result in multiplicative decreases.
        let multiplicative_zoom = 1. + delta_zoom;

        orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(0.01, 100.0);
    }
}

#[derive(Component)]
pub struct VisualPosition(Vec3);

impl VisualPosition {
    pub fn new() -> Self {
        VisualPosition(Vec3::ZERO)
    }

    pub fn set(&mut self, pos: Vec3) {
        self.0 = pos;
    }

    pub fn get(&self) -> Vec3 {
        self.0
    }
}

pub fn interpolate_player_position(
    mut query: Query<(&Transform, &mut VisualPosition), With<Player>>,
    time: Res<Time>,
) {
    for (transform, mut visual) in &mut query {
        // tweak factor под плавность
        let lerp_factor = 12.0 * time.delta_secs();
        visual.0 = visual
            .0
            .lerp(transform.translation, lerp_factor.clamp(0.0_f32, 1.0_f32));
    }
}
