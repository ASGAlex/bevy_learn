use std::marker::PhantomData;

use bevy::prelude::*;

pub struct PoolPlugin<T: Component> {
    initial_size: usize,
    spawn_fn: fn(&mut Commands) -> Entity,
    _marker: PhantomData<T>,
}

impl<T: Component> PoolPlugin<T> {
    pub fn new(initial_size: usize, spawn_fn: fn(&mut Commands) -> Entity) -> Self {
        Self {
            initial_size,
            spawn_fn,
            _marker: PhantomData,
        }
    }
}

impl<T: Component> Plugin for PoolPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pool<T>>();
        let spawn_fn = self.spawn_fn;

        let initial_size = self.initial_size;

        app.world_mut()
            .resource_scope(|world, mut pool: Mut<Pool<T>>| {
                pool.spawn_fn.get_or_insert(spawn_fn);

                extend_pool(&mut world.commands(), &mut pool, initial_size);
            });
    }
}

#[derive(Component)]
pub struct Inactive<T: Component>(pub PhantomData<T>);

#[derive(Component)]
pub struct Active<T: Component>(pub PhantomData<T>);

#[derive(Resource, Reflect)]
pub struct Pool<T: Component> {
    pub free: Vec<Entity>,
    #[reflect(ignore)]
    spawn_fn: Option<fn(&mut Commands) -> Entity>,
    #[reflect(ignore)]
    _marker: PhantomData<T>,
}

impl<T: Component> Default for Pool<T> {
    fn default() -> Self {
        Self {
            free: Vec::new(),
            _marker: PhantomData,
            spawn_fn: None,
        }
    }
}

pub fn extend_pool<T: Component>(commands: &mut Commands, pool: &mut Pool<T>, count: usize) {
    let Some(spawn_fn) = pool.spawn_fn else {
        return;
    };

    for _ in 0..count {
        let e = spawn_fn(commands);

        commands
            .entity(e)
            .insert((Visibility::Hidden, Inactive::<T>(PhantomData)));

        pool.free.push(e);
        dbg!(format!("Adding {} bullets", count));
    }
}

pub fn activate_from_pool<T: Component>(
    commands: &mut Commands,
    pool: &mut Pool<T>,
    activate_fn: impl FnOnce(Entity, &mut Commands),
) -> bool {
    if pool.free.is_empty() {
        extend_pool(commands, pool, 100);
    }
    let Some(entity) = pool.free.pop() else {
        return false;
    };

    // dbg!(("Activate: ", entity));
    commands
        .entity(entity)
        .remove::<Inactive<T>>()
        .insert((Visibility::Visible, Active::<T>(PhantomData)));

    activate_fn(entity, commands);
    true
}

pub fn deactivate_to_pool<T: Component>(
    commands: &mut Commands,
    pool: &mut Pool<T>,
    entity: Entity,
    deactivate_fn: impl FnOnce(Entity, &mut Commands),
) {
    deactivate_fn(entity, commands);

    commands
        .entity(entity)
        .remove::<Active<T>>()
        .insert((Visibility::Hidden, Inactive::<T>(PhantomData)));

    pool.free.push(entity);
    // dbg!(("DEActivate: ", entity));
}
