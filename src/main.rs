mod objects;
use crate::objects::{GameObject, ObjectsCounter};
use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ObjectsCounter::new())
        .add_systems(Startup, init)
        .add_systems(Update, hello_world)
        .run();
}

fn hello_world(mut query: Query<&mut GameObject>) {
    for mut object in query.iter_mut() {
        object.do_logic();
        object.name();
    }
}

fn init(mut commands: Commands, mut counter: ResMut<ObjectsCounter>) {
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
    commands.spawn(GameObject::new(&mut counter));
}
