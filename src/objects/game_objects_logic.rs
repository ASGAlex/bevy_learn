use crate::objects::GameObject;
use rand::random;

impl GameObject {
    fn rename(&mut self, name: String) {
        self.name = name;
    }

    pub fn random_rename(&mut self) {
        let random = random::<u32>();
        if !(random > u32::MAX / 3) {
            return;
        }
        self.rename("Renamed: ".to_string() + &random.to_string());
    }
}
