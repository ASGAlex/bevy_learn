use crate::{
    game::{actors::movement::PlayerMoving, map_objects::GameLayer},
    utils::tileset_reader::read_sprite_animation_from_tileset,
};
use avian2d::prelude::{
    AngularDamping, Collider, CollidingEntities, CollisionEventsEnabled, CollisionLayers,
    LinearDamping, LockedAxes, RigidBody,
};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;
use bevy_spritesheet_animation::prelude::{Animation, SpritesheetAnimation};

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    transform: Transform,
    collision_data: CollisionDataBundle,
}

impl PlayerBundle {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            player: Player,
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
    locked_axes: LockedAxes,
}

impl CollisionDataBundle {
    pub fn new() -> Self {
        Self {
            rigid_body: RigidBody::Dynamic,
            collider: Collider::round_rectangle(12.0, 12.0, 1.5),
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
            locked_axes: LockedAxes::ROTATION_LOCKED,
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    tiled_map_assets: Res<Assets<TiledMapAsset>>,
    existing_player: Option<Single<Entity, With<Player>>>,
    animations: ResMut<Assets<Animation>>,
    atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    images: ResMut<Assets<Image>>,
) {
    if existing_player.is_some() {
        return;
    }

    let Some((sprite, animation)) = read_sprite_animation_from_tileset(
        "tank".to_string(),
        0,
        tiled_map_assets,
        animations,
        atlas_layouts,
        images,
    ) else {
        return;
    };

    //===================
    // !!! Example usage:
    //
    // let Some(sprite) =
    //     read_sprite_from_tileset("tank".to_string(), 0, tiled_map_assets, atlas_layouts)
    // else {
    //     return;
    // };
    let id = commands.spawn(PlayerBundle::new(0.0, 0.0, -150.0)).id();
    //commands.entity(id).insert(TileDestructor::default());
    commands.entity(id).insert((
        sprite,
        animation,
        LinearDamping(10.0),
        AngularDamping(0.0),
        //        MaxLinearSpeed(50.0),
    ));
}

pub fn player_animation_controller(
    moving: Res<PlayerMoving>,
    mut query: Query<&mut SpritesheetAnimation, With<Player>>,
) {
    // если состояние не менялось — вообще ничего не делаем
    if !moving.is_changed() {
        return;
    }

    for mut animation in query.iter_mut() {
        if moving.is_moving {
            animation.play();
        } else {
            animation.pause();
            animation.reset(); // опционально: вернуться в idle-фрейм
        }
    }
}
