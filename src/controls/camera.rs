use bevy::camera::{Camera2d, Projection};
use bevy::input::mouse::AccumulatedMouseScroll;
use bevy::math::Vec3;
use bevy::prelude::{Res, Single, StableInterpolate, Time, Transform, With, Without};
use crate::{MainCamera, CAMERA_DECAY_RATE};
use crate::player::player::Player;

pub fn update_camera(
    mut camera: Single<&mut Transform, (With<Camera2d>, With<MainCamera>)>,
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    time: Res<Time>,
) {
    let Vec3 { x, y, .. } = player.translation;
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
    match projection.into_inner().as_mut() {
        Projection::Orthographic(orthographic) => {
            // We want scrolling up to zoom in, decreasing the scale, so we negate the delta.
            let delta_zoom = -mouse_wheel_input.delta.y * 0.2;
            // When changing scales, logarithmic changes are more intuitive.
            // To get this effect, we add 1 to the delta, so that a delta of 0
            // results in no multiplicative effect, positive values result in a multiplicative increase,
            // and negative values result in multiplicative decreases.
            let multiplicative_zoom = 1. + delta_zoom;

            orthographic.scale = (orthographic.scale * multiplicative_zoom).clamp(0.01, 100.0);
        }
        _ => (),
    }
}
