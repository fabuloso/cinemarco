use crate::screening::ScreeningSchedule;
use std::collections::HashMap;
pub struct Schedules {
    pub screenings: HashMap<u32, ScreeningSchedule>,
}

impl Schedules {
    pub fn with(&mut self, id: u32) -> Option<&ScreeningSchedule> {
        self.screenings.get(&id)
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
