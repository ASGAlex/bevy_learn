use bevy::prelude::Resource;

#[derive(Resource)]
pub struct ObjectsCounter(u32);

impl ObjectsCounter {
    pub fn new() -> Self {
        ObjectsCounter(0)
    }

    pub fn increment(&mut self) -> u32 {
        self.0 += 1;
        self.0
    }
}