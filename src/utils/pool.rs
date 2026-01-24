use std::marker::PhantomData;

use bevy::prelude::*;

pub struct PoolPlugin<T: Component> {
    _marker: PhantomData<T>,
}

impl<T: Component> Default for PoolPlugin<T> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<T: Component> Plugin for PoolPlugin<T> {
    fn build(&self, app: &mut App) {
        app.init_resource::<Pool<T>>();
    }
}

#[derive(Component)]
pub struct Inactive<T: Component>(pub PhantomData<T>);

#[derive(Component)]
pub struct Active<T: Component>(pub PhantomData<T>);

#[derive(Resource)]
pub struct Pool<T: Component> {
    pub free: Vec<Entity>,
    _marker: PhantomData<T>,
}

impl<T: Component> Default for Pool<T> {
    fn default() -> Self {
        Self {
            free: Vec::new(),
            _marker: PhantomData,
        }
    }
}

pub fn setup_pool<T: Component>(
    commands: &mut Commands,
    pool: &mut Pool<T>,
    count: usize,
    mut spawn_fn: impl FnMut(&mut Commands) -> Entity,
) {
    for _ in 0..count {
        let e = spawn_fn(commands);

        commands
            .entity(e)
            .insert((Visibility::Hidden, Inactive::<T>(PhantomData)));

        pool.free.push(e);
    }
    dbg!("Pool size is:".to_string() + &pool.free.len().to_string());
}

pub fn activate_from_pool<T: Component>(
    commands: &mut Commands,
    pool: &mut Pool<T>,
    activate_fn: impl FnOnce(Entity, &mut Commands),
) -> bool {
    let Some(entity) = pool.free.pop() else {
        return false;
    };

    dbg!(("Activate: ", entity));
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
    dbg!(("DEActivate: ", entity));
}
