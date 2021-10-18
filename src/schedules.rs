use std::collections::HashMap;

use crate::screening::ScreeningSchedule;
pub struct Schedules {
    pub screenings: HashMap<u32, ScreeningSchedule>,
}

impl Schedules {
    pub fn with(&mut self, id: u32) -> Option<&mut ScreeningSchedule> {
        self.screenings.get_mut(&id)
    }

    pub fn new() -> Self {
        Self {
            screenings: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: u32, screening: ScreeningSchedule) {
        self.screenings.insert(id, screening);
    }
}
