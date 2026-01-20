use crate::PLAYER_SPEED;
use crate::player::player::Player;
use avian2d::prelude::LinearVelocity;
use bevy::ecs::resource::Resource;
use bevy::ecs::system::ResMut;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Query, Res, With};

#[derive(Resource, Default, Clone, Copy)]
pub struct LastMoveDir(pub Option<Vec2>);

pub fn move_player(
    mut player: Query<&mut LinearVelocity, With<Player>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut last_dir: ResMut<LastMoveDir>,
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

    // применяем скорость
    for mut vel in player.iter_mut() {
        vel.0 = direction * PLAYER_SPEED;
    }
}
