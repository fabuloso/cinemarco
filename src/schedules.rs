use crate::screening::ScreeningSchedule;
pub struct Schedules {
    pub screenings: Vec<ScreeningSchedule>,
}

impl Schedules {
    pub fn with(&mut self, id: u32) -> Option<&ScreeningSchedule> {
        self.screenings.get(id as usize)
    }

    pub fn new() -> Self {
        Self { screenings: vec![] }
    }

    pub fn add(&mut self, id: u32, screening: ScreeningSchedule) {
        self.screenings.insert(id as usize, screening);
    }
}
