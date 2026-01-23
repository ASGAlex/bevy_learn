use crate::PLAYER_SPEED;
use crate::player::player::Player;
use avian2d::prelude::LinearVelocity;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::ResMut;
use bevy::input::ButtonInput;
use bevy::math::{Quat, Vec2};
use bevy::prelude::{KeyCode, Query, Res, With};
use bevy::sprite::Sprite;
use bevy::transform::components::Transform;

#[derive(Resource, Default, Clone, Copy, PartialEq, Eq)]
pub struct PlayerMoving {
    pub is_moving: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookDir {
    Up,
    Down,
    Left,
    Right,
}

impl LookDir {
    pub fn to_vec2(self) -> Vec2 {
        match self {
            LookDir::Up => Vec2::Y,
            LookDir::Down => -Vec2::Y,
            LookDir::Left => -Vec2::X,
            LookDir::Right => Vec2::X,
        }
    }
}

#[derive(Resource, Default, Clone, Copy)]
pub struct PlayerLookDir(pub Option<LookDir>);

#[derive(Resource, Default, Clone, Copy)]
pub struct LastMoveDir(pub Option<Vec2>);

pub fn move_player(
    mut player: Query<&mut LinearVelocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut last_dir: ResMut<LastMoveDir>,
    mut look_dir: ResMut<PlayerLookDir>,
    mut moving: ResMut<PlayerMoving>,
) {
    // обновляем направление только по just_pressed (последняя клавиша важнее)
    if kb_input.just_pressed(KeyCode::KeyW) {
        last_dir.0 = Some(Vec2::Y);
    }
    if kb_input.just_pressed(KeyCode::KeyS) {
        last_dir.0 = Some(-Vec2::Y);
    }
    if kb_input.just_pressed(KeyCode::KeyA) {
        last_dir.0 = Some(-Vec2::X);
    }
    if kb_input.just_pressed(KeyCode::KeyD) {
        last_dir.0 = Some(Vec2::X);
    }

    // проверяем: всё ещё зажата ли последняя клавиша
    let mut direction = Vec2::ZERO;

    if let Some(dir) = last_dir.0 {
        let still_pressed = match dir {
            d if d == Vec2::Y => kb_input.pressed(KeyCode::KeyW),
            d if d == -Vec2::Y => kb_input.pressed(KeyCode::KeyS),
            d if d == -Vec2::X => kb_input.pressed(KeyCode::KeyA),
            d if d == Vec2::X => kb_input.pressed(KeyCode::KeyD),
            _ => false,
        };

        if still_pressed {
            direction = dir;
        } else {
            // последнюю отпустили — ищем другую нажатую
            if kb_input.pressed(KeyCode::KeyW) {
                direction = Vec2::Y;
                last_dir.0 = Some(direction);
            } else if kb_input.pressed(KeyCode::KeyS) {
                direction = -Vec2::Y;
                last_dir.0 = Some(direction);
            } else if kb_input.pressed(KeyCode::KeyA) {
                direction = -Vec2::X;
                last_dir.0 = Some(direction);
            } else if kb_input.pressed(KeyCode::KeyD) {
                direction = Vec2::X;
                last_dir.0 = Some(direction);
            } else {
                last_dir.0 = None;
            }
        }
    }

    // если реально движемся — обновляем направление взгляда
    if direction != Vec2::ZERO {
        look_dir.0 = Some(match direction {
            d if d == Vec2::Y => LookDir::Up,
            d if d == -Vec2::Y => LookDir::Down,
            d if d == -Vec2::X => LookDir::Left,
            d if d == Vec2::X => LookDir::Right,
            _ => unreachable!(),
        });
    }

    let now_moving = direction != Vec2::ZERO;
    moving.is_moving = now_moving;
    // применяем скорость

    for mut vel in player.iter_mut() {
        if direction == Vec2::ZERO {
            vel.0 = Vec2::ZERO; // <- ВАЖНО
        } else {
            vel.0 = direction * PLAYER_SPEED;
        }
    }
}

pub fn apply_player_look_dir(
    look_dir: Res<PlayerLookDir>,
    mut player: Query<&mut Transform, With<Player>>,
) {
    let Some(dir) = look_dir.0 else { return };

    let rotation = match dir {
        LookDir::Up => 0.0,
        LookDir::Right => -std::f32::consts::FRAC_PI_2,
        LookDir::Down => std::f32::consts::PI,
        LookDir::Left => std::f32::consts::FRAC_PI_2,
    };

    for mut transform in player.iter_mut() {
        transform.rotation = Quat::from_rotation_z(rotation);
    }
}
