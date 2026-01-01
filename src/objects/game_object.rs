use crate::objects::ObjectsCounter;
use bevy::prelude::Component;

#[derive(Component)]
pub struct GameObject {
    pub(in crate::objects) name: String,
}

impl GameObject {
    pub fn new(counter: &mut ObjectsCounter) -> Self {
        let number = counter.increment();
        Self {
            name: String::from("Object #".to_string() + &number.to_string()),
        }
    }

    pub fn name(&self) {
        println!("{}", self.name);
    }

    pub fn do_logic(&mut self) {
        self.random_rename();
    }
}
