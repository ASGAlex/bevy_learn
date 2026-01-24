use avian2d::prelude::*;
use bevy::app::{Plugin, Update};
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy_ecs_tiled::physics::RemovedTilesStorage;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tiled::tiled::event::TilemapUpdatedMarker;

#[derive(Default)]
pub struct TileDestructorPlugin;

impl Plugin for TileDestructorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, destructor_remove_tiles);
    }
}

#[allow(dead_code)]
#[derive(Component)]
pub struct TileDestructor;

#[allow(dead_code)]
#[derive(Component)]
pub struct AffectedByDestructor;

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub fn destructor_remove_tiles(
    mut commands: Commands,
    q_destructor: Query<Entity, With<TileDestructor>>,
    q_maps: Query<(&TiledMap, &TiledMapStorage), Without<RespawnTiledMap>>,
    mut q_tiled_tilemap: Query<
        (
            Entity,
            &ChildOf,
            &TilemapSize,
            &TilemapGridSize,
            &TilemapTileSize,
            &TilemapType,
            &mut TileStorage,
            &GlobalTransform,
            &TilemapAnchor,
            &TiledMapReference,
            Option<&TilemapUpdatedMarker>,
        ),
        With<AffectedByDestructor>,
    >,
    q_colliders: Query<&ChildOf, With<Collider>>,
    collisions: Collisions,
    mut removed_tiles: ResMut<RemovedTilesStorage>,
    map_assets: Res<Assets<TiledMapAsset>>,
) {
    for destructor_entity in q_destructor {
        for collision in collisions.collisions_with(destructor_entity) {
            let point = match collision.find_deepest_contact() {
                Some(point) => point,
                None => continue,
            };

            let collider_entity = if let Some(body1) = collision.body1 {
                if body1 == destructor_entity {
                    collision.collider2
                } else {
                    collision.collider1
                }
            } else {
                continue;
            };

            let parent_of_collider = match q_colliders.get(collider_entity) {
                Ok(parent) => parent,
                Err(_) => continue,
            };

            for (
                tilemap,
                parent_of_layer,
                &size,
                &grid,
                &tile,
                &map_type,
                mut storage,
                &transform,
                &anchor,
                map_reference,
                is_updated,
            ) in q_tiled_tilemap.iter_mut()
            {
                if parent_of_collider != parent_of_layer {
                    continue;
                }

                let pos = match get_tile_pos_from_world_pos(
                    transform,
                    point.point,
                    size,
                    grid,
                    tile,
                    map_type,
                    anchor,
                ) {
                    Some(pos) => pos,
                    None => continue,
                };

                if let Some(tile_entity) = storage.get(&pos) {
                    commands.entity(tile_entity).despawn();
                    storage.remove(&pos);

                    if let Ok((map_handle, map_storage)) = q_maps.get(map_reference.entity()) {
                        if let Some(map_asset) = map_assets.get(map_handle.0.id()) {
                            let Some(layer_id) = map_storage.get_layer_id(parent_of_collider.get())
                            else {
                                continue;
                            };
                            removed_tiles.add_tile(map_asset, layer_id, &pos);
                        }
                    }

                    commands.entity(collider_entity).despawn();

                    commands
                        .entity(tilemap)
                        .insert_if(TilemapUpdatedMarker, || is_updated.is_none());

                    break;
                }
            }
        }
    }
}

pub fn get_tile_pos_from_world_pos(
    transform: GlobalTransform,
    world_point: Vec2,
    size: TilemapSize,
    grid: TilemapGridSize,
    tile: TilemapTileSize,
    map_type: TilemapType,
    anchor: TilemapAnchor,
) -> Option<TilePos> {
    let transformed_point =
        (transform.to_matrix().inverse() * Vec4::from((world_point, 0., 1.))).xy();

    TilePos::from_world_pos(
        &Vec2::new(transformed_point.x, transformed_point.y),
        &size,
        &grid,
        &tile,
        &map_type,
        &anchor,
    )
}
