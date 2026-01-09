use avian2d::prelude::LinearVelocity;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Query, Res, Time, With};
use crate::player::player::Player;
use crate::PLAYER_SPEED;

pub fn move_player(
    mut player: Query<&mut LinearVelocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    for mut vel in player.iter_mut() {
        let mut direction = Vec2::ZERO;

        if kb_input.pressed(KeyCode::KeyW) {
            direction.y += 1.;
        }

        if kb_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.;
        }

        if kb_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.;
        }

        if kb_input.pressed(KeyCode::KeyD) {
            direction.x += 1.;
        }

        vel.0 = direction * PLAYER_SPEED;
    }
}
