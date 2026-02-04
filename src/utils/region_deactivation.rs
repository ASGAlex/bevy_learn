use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{MAP_CHUNK_SIZE, game::actors::player::Player};
pub struct RegionActivationPlugin;

impl Plugin for RegionActivationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (region_deactivate_system, region_activate_system),
        );
    }
}
#[derive(Component)]
pub struct RegionAware;

#[derive(Component)]
pub struct Deactivated;

fn region_deactivate_system(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    query: Query<
        (Entity, &Transform, Option<&RigidBody>),
        (With<RegionAware>, Without<Deactivated>),
    >,
) {
    let player_pos = player.translation;

    for (entity, transform, rb) in query.iter() {
        let diff = (player_pos - transform.translation).abs();

        if diff.x >= MAP_CHUNK_SIZE || diff.y >= MAP_CHUNK_SIZE {
            let mut e = commands.entity(entity);
            e.insert(Deactivated);

            if rb.is_some() {
                commands.queue(SleepBody(entity));
            }
        }
    }
}

fn region_activate_system(
    mut commands: Commands,
    player: Single<&Transform, With<Player>>,
    query: Query<(Entity, &Transform, Option<&RigidBody>), (With<RegionAware>, With<Deactivated>)>,
) {
    let player_pos = player.translation;

    for (entity, transform, rb) in query.iter() {
        let diff = (player_pos - transform.translation).abs();

        if diff.x < MAP_CHUNK_SIZE && diff.y < MAP_CHUNK_SIZE {
            let mut e = commands.entity(entity);
            e.remove::<Deactivated>();

            if rb.is_some() {
                commands.queue(WakeBody(entity));
            }
        }
    }
}
