use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::{
    controls::{LookDir, PlayerLookDir},
    player::Player,
};
pub const PLAYER_SIZE: Vec2 = Vec2::new(14.0, 14.0);
#[derive(Component)]
pub struct Bullet {
    pub dir: Vec2,
    pub speed: f32,
    pub traveled: f32,
    pub max_distance: f32,
}

pub fn shoot_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    look_dir: Res<PlayerLookDir>,
    player_q: Single<&Transform, With<Player>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Some(dir) = look_dir.0 else { return };

    let dir_vec = dir.to_vec2();
    let spawn_offset = bullet_spawn_offset(dir, PLAYER_SIZE);
    commands.spawn((
        Sprite {
            color: Color::srgb(0.9, 0.9, 0.9),
            custom_size: Some(Vec2::splat(2.0)),
            ..default()
        },
        Transform::from_translation(player_q.translation + spawn_offset),
        RigidBody::Dynamic,
        LinearVelocity(dir_vec * 100.0),
        Collider::circle(4.0),
        Bullet {
            dir: dir_vec,
            speed: 600.0, // пикселей в секунду
            traveled: 0.0,
            max_distance: 800.0, // дальность
        },
    ));
}

pub fn bullet_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &LinearVelocity, &mut Bullet)>,
) {
    for (entity, vel, mut bullet) in bullets.iter_mut() {
        bullet.traveled += vel.0.length() * time.delta_secs();

        if bullet.traveled >= bullet.max_distance {
            commands.entity(entity).despawn();
        }
    }
}

fn bullet_spawn_offset(dir: LookDir, player_size: Vec2) -> Vec3 {
    let half = player_size / 2.0;

    match dir {
        LookDir::Up => Vec3::new(0.0, half.y, 0.0),
        LookDir::Down => Vec3::new(0.0, -half.y, 0.0),
        LookDir::Left => Vec3::new(-half.x, 0.0, 0.0),
        LookDir::Right => Vec3::new(half.x, 0.0, 0.0),
    }
}
