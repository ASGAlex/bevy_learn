use avian2d::prelude::PhysicsLayer;

pub mod actors;
pub mod map_objects;
pub mod map_tiles;
pub mod weapons;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Ground,
    Water,
    Player,
    Bricks,
    Trees,
    Sky,
}
