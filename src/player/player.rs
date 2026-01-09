use crate::GameLayer;
use avian2d::prelude::{
    Collider, CollidingEntities, CollisionEventsEnabled, CollisionLayers, RigidBody,
};
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    mesh: Mesh2d,
    material: MeshMaterial2d<ColorMaterial>,
    transform: Transform,
    collision_data: CollisionDataBundle,
}

impl PlayerBundle {
    pub fn new(
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        x: f32,
        y: f32,
        z: f32,
    ) -> Self {
        Self {
            player: Player,
            mesh: Mesh2d(meshes.add(Circle::new(10.))),
            material: MeshMaterial2d(materials.add(Color::srgb(6.25, 9.4, 9.1))),
            transform: Transform::from_xyz(x, y, z),
            collision_data: CollisionDataBundle::new(),
        }
    }
}

#[derive(Bundle)]
pub struct CollisionDataBundle {
    rigid_body: RigidBody,
    collider: Collider,
    collision_events_enabled: CollisionEventsEnabled,
    colliding_entities: CollidingEntities,
    layers: CollisionLayers,
}

impl CollisionDataBundle {
    pub fn new() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::circle(4.),
            collision_events_enabled: CollisionEventsEnabled,
            colliding_entities: CollidingEntities::default(),
            layers: CollisionLayers::new(
                GameLayer::Player,
                [
                    GameLayer::Water,
                    GameLayer::Player,
                    GameLayer::Bricks,
                    GameLayer::Trees,
                ],
            ),
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(PlayerBundle::new(
        &mut meshes,
        &mut materials,
        0.0,
        0.0,
        -150.0,
    ));
}
