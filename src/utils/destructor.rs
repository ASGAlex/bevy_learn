use avian2d::prelude::*;
use bevy::app::Plugin;
use bevy::ecs::relationship::Relationship;
use bevy::prelude::*;
use bevy_ecs_tiled::physics::RemovedTilesStorage;
use bevy_ecs_tiled::prelude::*;
use bevy_ecs_tiled::tiled::event::TilemapUpdatedMarker;

use crate::game::weapons::bullet::Bullet;
use crate::utils::pool::Pool;

#[derive(Default)]
pub struct TileDestructorPlugin;

impl Plugin for TileDestructorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(FixedPreUpdate, destructor_remove_tiles);
    }
}

#[allow(dead_code)]
#[derive(Component, Reflect)]
pub struct TileDestructor<P: Component + Send + Sync + 'static> {
    pub remove_on_contact: bool,
    pub vector: Vec2,
    #[reflect(ignore)]
    pub remove_fn: Option<fn(&mut Commands, Entity, &mut Pool<P>)>,
}

impl<P: Component + Send + Sync + 'static> Default for TileDestructor<P> {
    fn default() -> Self {
        Self {
            remove_on_contact: false,
            vector: Vec2::ZERO,
            remove_fn: None,
        }
    }
}

#[derive(Component)]
struct TileDestructorRemoveMarker;

#[allow(dead_code)]
#[derive(Component, Default)]
pub struct AffectedByDestructor {
    pub layer_id: u32,
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
fn destructor_remove_tiles(
    mut commands: Commands,
    q_destructor: Query<(Entity, &TileDestructor<Bullet>), With<TileDestructor<Bullet>>>,
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
            &AffectedByDestructor,
            Option<&TilemapUpdatedMarker>,
        ),
        With<AffectedByDestructor>,
    >,
    q_colliders: Query<&ChildOf, With<Collider>>,
    collisions: Collisions,
    mut removed_tiles: ResMut<RemovedTilesStorage>,
    map_assets: Res<Assets<TiledMapAsset>>,
    mut pool: ResMut<Pool<Bullet>>,
) {
    for (destructor_entity, destructor_config) in q_destructor {
        for collision in collisions.collisions_with(destructor_entity) {
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

            for manifold in &collision.manifolds {
                for point in &manifold.points {
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
                        affected_by_destructor,
                        is_updated,
                    ) in q_tiled_tilemap.iter_mut()
                    {
                        if parent_of_collider != parent_of_layer {
                            continue;
                        }

                        // let pentration_point = destructor_config
                        //     .vector
                        //     .mul_add(Vec2 { x: 3.0, y: 3.0 }, point.point);

                        let penteration_points =
                            PenetrationPoints::new(point.point, destructor_config.vector, 1.5);

                        for penteration_point in penteration_points {
                            let pos = match get_tile_pos_from_world_pos(
                                transform,
                                penteration_point,
                                size,
                                grid,
                                tile,
                                map_type,
                                anchor,
                            ) {
                                Some(pos) => pos,
                                None => continue,
                            };

                            // dbg!((
                            //     pos,
                            //     destructor_config.vector,
                            //     point.point,
                            //     penteration_point
                            // ));

                            if let Some(tile_entity) = storage.get(&pos) {
                                commands.entity(tile_entity).despawn();
                                storage.remove(&pos);

                                if let Ok(map_handle) = q_maps.get(map_reference.entity())
                                    && let Some(map_asset) = map_assets.get(map_handle.0.id())
                                {
                                    removed_tiles.add_tile(
                                        map_asset,
                                        affected_by_destructor.layer_id,
                                        &pos,
                                    );
                                }

                                commands.entity(collider_entity).despawn();

                                commands.entity(tilemap).insert_if(
                                    TilemapUpdatedMarker {
                                        layer_id: affected_by_destructor.layer_id,
                                    },
                                    || is_updated.is_none(),
                                );

                                if destructor_config.remove_on_contact {
                                    match destructor_config.remove_fn {
                                        Some(function) => {
                                            function(&mut commands, destructor_entity, &mut pool)
                                        }
                                        None => commands.entity(destructor_entity).despawn(),
                                    }
                                }

                                //break;
                            }
                        }
                    }
                }
            }
        }
    }
}

fn get_tile_pos_from_world_pos(
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

struct PenetrationPoints {
    point: Vec2,
    direction: Vec2,
    right: Vec2,
    left: Vec2,
    depth: Vec2,
    index: usize,
}

impl PenetrationPoints {
    fn new(point: Vec2, direction: Vec2, depth: f32) -> Self {
        const COS_45: f32 = 0.707;
        let (left, right) = match (direction.x as i32, direction.y as i32) {
            // Направление → ВПРАВО (1, 0)
            (1, 0) => (
                Vec2 {
                    x: COS_45,
                    y: COS_45,
                }, // влево: 45° вверх-вправо
                Vec2 {
                    x: COS_45,
                    y: -COS_45,
                }, // вправо: 45° вниз-вправо
            ),
            // Направление → ВЛЕВО (-1, 0)
            (-1, 0) => (
                Vec2 {
                    x: -COS_45,
                    y: -COS_45,
                }, // влево: 45° вниз-влево
                Vec2 {
                    x: -COS_45,
                    y: COS_45,
                }, // вправо: 45° вверх-влево
            ),
            // Направление → ВВЕРХ (0, 1)
            (0, 1) => (
                Vec2 {
                    x: -COS_45,
                    y: COS_45,
                }, // влево: 45° вверх-влево
                Vec2 {
                    x: COS_45,
                    y: COS_45,
                }, // вправо: 45° вверх-вправо
            ),
            // Направление → ВНИЗ (0, -1)
            (0, -1) => (
                Vec2 {
                    x: COS_45,
                    y: -COS_45,
                }, // влево: 45° вниз-вправо
                Vec2 {
                    x: -COS_45,
                    y: -COS_45,
                }, // вправо: 45° вниз-влево
            ),
            // На случай нештатного ввода (хотя по условию не бывает)
            _ => (Vec2::ZERO, Vec2::ZERO),
        };
        let depth_vector = Vec2::new(depth, depth);

        Self {
            point,
            direction,
            right,
            left,
            depth: depth_vector,
            index: 0,
        }
    }
}
impl Iterator for PenetrationPoints {
    type Item = Vec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                // FMA: (direction * depth) + point
                Some(self.direction.mul_add(self.depth, self.point))
            }
            1 => {
                self.index += 1;
                // FMA: (right_perp * depth) + point
                Some(self.right.mul_add(self.depth, self.point))
            }
            2 => {
                self.index += 1;
                // FMA: (left_perp * depth) + point
                Some(self.left.mul_add(self.depth, self.point))
            }
            _ => None,
        }
    }
}
