use avian2d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::{
    game::actors::{
        movement::{LookDir, PlayerLookDir},
        player::Player,
    },
    utils::{destructor::TileDestructor, pool::*},
};
pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_system)
            .add_systems(Update, bullet_lifetime_system);
    }
}

pub const PLAYER_SIZE: Vec2 = Vec2::new(14.0, 14.0);

#[derive(Component)]
pub struct Bullet;

#[derive(Component)]
pub struct BulletData {
    pub traveled: f32,
    pub max_distance: f32,
}

pub fn shoot_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    look_dir: Res<PlayerLookDir>,
    player_q: Single<&Transform, With<Player>>,
    mut pool: ResMut<Pool<Bullet>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let Some(dir) = look_dir.0 else { return };

    let dir_vec = dir.to_vec2();
    let spawn_offset = bullet_spawn_offset(dir, PLAYER_SIZE);

    activate_from_pool::<Bullet>(&mut commands, &mut pool, |entity, commands| {
        commands.entity(entity).insert((
            Transform::from_translation(player_q.translation + spawn_offset),
            LinearVelocity(dir_vec * 100.0),
            TileDestructor,
            BulletData {
                traveled: 0.0,
                max_distance: 800.0,
            },
        ));
    });
}

pub fn bullet_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    mut bullets: Query<(Entity, &LinearVelocity, &mut BulletData), With<Active<Bullet>>>,
    mut pool: ResMut<Pool<Bullet>>,
) {
    for (entity, vel, mut bullet) in bullets.iter_mut() {
        bullet.traveled += vel.0.length() * time.delta_secs();

        if bullet.traveled >= bullet.max_distance {
            deactivate_to_pool::<Bullet>(&mut commands, &mut pool, entity, |entity, commands| {
                commands
                    .entity(entity)
                    .insert(LinearVelocity(Vec2::ZERO))
                    .remove::<TileDestructor>();
            });
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

pub fn setup_bullets(mut commands: Commands, mut pool: ResMut<Pool<Bullet>>) {
    setup_pool::<Bullet>(&mut commands, &mut pool, 128, |commands| {
        commands
            .spawn((
                Sprite {
                    color: Color::srgb(0.9, 0.9, 0.9),
                    custom_size: Some(Vec2::splat(2.0)),
                    ..default()
                },
                RigidBody::Dynamic,
                LinearVelocity(Vec2::ZERO),
                Collider::circle(2.0),
                Bullet,
                BulletData {
                    traveled: 0.0,
                    max_distance: 0.0,
                },
            ))
            .id()
    });
}
