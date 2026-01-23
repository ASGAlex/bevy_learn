use std::sync::Arc;

use crate::{
    GameLayer,
    controls::{PlayerMoving, VisualPosition},
    tile_destructor::destructor::TileDestructor,
};
use avian2d::prelude::{
    AngularDamping, Collider, CollidingEntities, CollisionEventsEnabled, CollisionLayers,
    LinearDamping, LockedAxes, MaxLinearSpeed, RigidBody,
};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{
    tiled::{Error, Result, Tileset},
    *,
};
use bevy_spritesheet_animation::prelude::{Animation, Spritesheet, SpritesheetAnimation};

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
            collider: Collider::circle(8.),
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
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    images: ResMut<Assets<Image>>,
) {
    if existing_player.is_some() {
        return;
    }

    let mut tileset_option: Option<&Arc<Tileset>> = None;
    let mut handle_option: Option<&Handle<Image>> = None;
    for (_, asset) in tiled_map_assets.iter() {
        if asset.map.tilesets().is_empty() {
            continue;
        }

        for ts in asset.map.tilesets() {
            if ts.name == "tank" {
                tileset_option = Some(ts);
                break;
            }
        }

        if tileset_option.is_none() {
            continue;
        }

        for (name, tileset) in asset.tilesets.iter() {
            if name.contains("tank") {
                let handlers = tileset.tilemap_texture.image_handles();
                handle_option = Some(*handlers.first().unwrap());
                break;
            }
        }

        if handle_option.is_none() {
            continue;
        }

        break;
    }

    if tileset_option.is_none() || handle_option.is_none() {
        return;
    }

    let tileset = tileset_option.unwrap();

    let Some(tile) = tileset.get_tile(0) else {
        return;
    };

    let Some(animation) = &tile.animation else {
        return;
    };

    let Some(handle) = handle_option else {
        return;
    };

    let rows = calculate_rows(
        &tileset.image,
        tileset.tile_height,
        tileset.margin,
        tileset.spacing,
    )
    .unwrap();

    let spritesheet = Spritesheet::new(handle, tileset.columns as usize, rows as usize);
    let sprite = spritesheet
        .with_loaded_image(&images)
        .expect("")
        .sprite(&mut atlas_layouts);
    let mut animation_builder = spritesheet.create_animation();

    for frame in animation {
        animation_builder = animation_builder
            .add_indices([frame.tile_id as usize])
            .set_clip_duration(
                bevy_spritesheet_animation::prelude::AnimationDuration::PerFrame(frame.duration),
            );
    }

    let final_animation = animation_builder.build();

    let animation_handle = animations.add(final_animation);

    let id = commands.spawn(PlayerBundle::new(0.0, 0.0, -150.0)).id();
    commands.entity(id).insert(TileDestructor);
    commands.entity(id).insert((
        sprite,
        SpritesheetAnimation::new(animation_handle),
        LinearDamping(10.0),
        AngularDamping(0.0),
        VisualPosition::new(),
        MaxLinearSpeed(50.0),
    ));
}

fn calculate_rows(
    image: &Option<bevy_ecs_tiled::prelude::tiled::Image>,
    tile_height: u32,
    margin: u32,
    spacing: u32,
) -> Result<u32> {
    image
        .as_ref()
        .map(|image| (image.height as u32 - margin + spacing) / (tile_height + spacing))
        .ok_or_else(|| {
            Error::MalformedAttributes("No <image> nor rows attribute in <tileset>".to_string())
        })
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
