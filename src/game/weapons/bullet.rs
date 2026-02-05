use avian2d::prelude::{
    AngularDamping, Collider, ColliderDisabled, CollisionLayers, Collisions, LinearDamping,
    LinearVelocity, LockedAxes, RigidBody, RigidBodyDisabled, Sleeping, SpeculativeMargin,
};
use bevy::prelude::*;

use crate::{
    GameLayer, MainCamera, PHYSICS_SPEED,
    game::actors::{
        movement::{LookDir, PlayerLookDir},
        player::Player,
    },
    utils::{destructor::TileDestructor, pool::*, region_deactivation::RegionAware},
};
pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, (shoot_system, bullet_lifetime_system))
            .add_plugins(PoolPlugin::<Bullet>::new(1, new_bullet))
            .init_resource::<ShootTimer>()
            .register_type::<Pool<Bullet>>()
            .register_type::<TileDestructor<Bullet>>();
    }
}

#[derive(Resource, Default)]
struct ShootTimer {
    last_shot: f32,
}

pub const PLAYER_SIZE: Vec2 = Vec2::new(14.0, 14.0);

#[derive(Component, Reflect)]
pub struct Bullet;

#[derive(Component, Clone, Copy)]
pub struct BulletData {
    pub traveled: f32,
    pub max_distance: f32,
    pub parent: Option<Entity>,
}

fn shoot_system(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    look_dir: Res<PlayerLookDir>,
    player_q: Single<(Entity, &Transform), With<Player>>,
    mut pool: ResMut<Pool<Bullet>>,
    time: Res<Time>,
    mut shoot_timer: ResMut<ShootTimer>,
) {
    if !keyboard.pressed(KeyCode::Space) {
        return;
    }

    let current_time = time.elapsed_secs();
    let cooldown = 0.5;
    if current_time - shoot_timer.last_shot < cooldown {
        return;
    }

    let Some(dir) = look_dir.0 else { return };

    let dir_vec = dir.to_vec2();
    let spawn_offset = bullet_spawn_offset(dir, PLAYER_SIZE);

    activate_from_pool::<Bullet>(&mut commands, &mut pool, |entity, commands| {
        let (player_entity, transform) = player_q.into_inner();

        commands
            .entity(entity)
            .insert((
                Transform::from_translation(transform.translation + spawn_offset),
                LinearVelocity(dir_vec * 800.0),
                TileDestructor::<Bullet> {
                    remove_on_contact: true,
                    vector: dir_vec,
                    remove_fn: Some(bullet_remove_on_contact),
                },
                BulletData {
                    traveled: 0.0,
                    max_distance: 1000.0 / PHYSICS_SPEED,
                    parent: Some(player_entity),
                },
            ))
            .remove::<ColliderDisabled>()
            .remove::<RigidBodyDisabled>();
    });

    shoot_timer.last_shot = current_time;
}

fn bullet_remove_on_contact(commands: &mut Commands, bullet: Entity, pool: &mut Pool<Bullet>) {
    deactivate_to_pool(commands, pool, bullet, |entity, commands| {
        commands
            .entity(entity)
            .insert((
                LinearVelocity(Vec2::ZERO),
                ColliderDisabled,
                RigidBodyDisabled,
            ))
            .remove::<TileDestructor<Bullet>>();
    });
}

pub fn bullet_lifetime_system(
    mut commands: Commands,
    time: Res<Time>,
    camera: Single<&Transform, (With<Camera2d>, With<MainCamera>)>,
    mut bullets: Query<
        (Entity, &LinearVelocity, &Transform, &mut BulletData),
        (With<Active<Bullet>>, Without<Sleeping>),
    >,
    collisions: Collisions,
    mut pool: ResMut<Pool<Bullet>>,
) {
    for (entity, vel, bullet_transform, mut bullet) in bullets.iter_mut() {
        bullet.traveled += vel.0.length() * time.delta_secs();

        let should_be_removed = bullet.traveled >= bullet.max_distance;

        if !should_be_removed {
            for collision in collisions.collisions_with(entity) {
                // Столкновение не с объектом, выпустившем пулю, а с чем-то ещё
                if collision.body1 != bullet.parent && collision.body2 != bullet.parent {
                    // should_be_removed = true;
                    break;
                }
            }
        }

        if should_be_removed {
            deactivate_to_pool::<Bullet>(&mut commands, &mut pool, entity, |entity, commands| {
                commands
                    .entity(entity)
                    .insert((
                        LinearVelocity(Vec2::ZERO),
                        ColliderDisabled,
                        RigidBodyDisabled,
                    ))
                    .remove::<TileDestructor<Bullet>>();
            });
        } else {
            // let diff = (camera.translation - bullet_transform.translation).abs();
            // if diff.x >= MAP_CHUNK_SIZE || diff.y >= MAP_CHUNK_SIZE {
            //     commands.entity(entity).insert((
            //         RigidBodyDisabled,
            //         ColliderDisabled,
            //         Visibility::Hidden,
            //     ));
            //     commands.queue(SleepBody(entity));
            //     dbg!("!!!!!!!!!!!!! SLEEP");
            // }
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

fn new_bullet(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Sprite {
                color: Color::srgb(0.9, 0.9, 0.9),
                custom_size: Some(Vec2::splat(2.0)),
                ..default()
            },
            RigidBody::Dynamic,
            LinearVelocity(Vec2::ZERO),
            Collider::rectangle(2.0, 2.0),
            Bullet,
            LockedAxes::ROTATION_LOCKED,
            LinearDamping(0.0),
            AngularDamping(0.0),
            // MaxLinearSpeed(10000.0),
            RigidBodyDisabled,
            ColliderDisabled,
            SpeculativeMargin(1.0),
            //SweptCcd::LINEAR,
            RegionAware,
            Visibility::Hidden,
            CollisionLayers::new(GameLayer::Player, [GameLayer::Player, GameLayer::Bricks]),
            BulletData {
                traveled: 0.0,
                max_distance: 0.0,
                parent: None,
            },
        ))
        .id()
}
