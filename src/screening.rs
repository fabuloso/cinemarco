pub struct ScreeningSchedule {
    id: u32,
    screening: Screening,
    room: Room,
}

pub struct Screening {
    pub movie: MovieId,
    pub room: RoomId,
    pub startTime: StartTime,
}

pub struct StartTime {
    pub minutes: u16,
    pub hours: u16,
    pub day: u16,
    pub month: u8,
    pub year: u16,
}

pub struct Movie {
    id: MovieId,
    title: String,
    subtitle: String,
    year: Year,
    cast: Vec<Actor>,
    director: Director,
    publisher: Publisher,
}

pub struct MovieId(u32);
pub struct Year(u32);
pub struct RoomId(u32);

pub struct Director {
    name: String,
    surname: String,
}

pub struct Publisher {
    name: String,
    surname: String,
}

pub struct Actor {
    name: String,
    surname: String,
}

pub struct Room {
    id: RoomId,
    name: String,
    seats: Vec<Seat>,
}

pub struct Seat {
    row: u16,
    number: u16,
    dbox: bool,
}
