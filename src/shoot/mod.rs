use bevy::app::{App, Plugin, Update};

use crate::shoot::bullet::*;

pub mod bullet;

pub struct ShootingPlugin;

impl Plugin for ShootingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, shoot_system)
            .add_systems(Update, bullet_lifetime_system);
    }
}
