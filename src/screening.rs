impl ScreeningSchedule {
    pub fn book(&mut self, row: u16, number: u16) {
        let free = self.room.is_free(row, number);
        if free {
            self.room.reserve_seat(row, number);
        }
    }
}
pub struct ScreeningSchedule {
    pub id: u32,
    pub screening: Screening,
    pub room: Room,
}

pub struct Screening {
    pub movie: MovieId,
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

pub struct MovieId {
    pub id: u32,
}
pub struct Year(u32);
pub struct RoomId {
    pub id: u32,
}

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
    pub id: RoomId,
    pub name: String,
    pub seats: Seats,
}

pub struct Seats {}

pub struct Seat {
    row: u16,
    number: u16,
    dbox: bool,
    reserved: bool,
}

impl Seat {
    fn is_already_reserved(&self) -> bool {
        self.reserved
    }
}

impl Seats {
    fn at(&self, row: u16, number: u16) -> Result<Seat, String> {
        Ok(Seat {
            row,
            number,
            dbox: false,
            reserved: false,
        })
    }
}

impl Room {
    fn is_free(&self, row: u16, number: u16) -> bool {
        let seat = self.seats.at(row, number);
        seat.unwrap().is_already_reserved()
    }

    fn reserve_seat(&mut self, row: u16, number: u16) {
        let mut seat = self.seats.at(row, number).unwrap();
        seat.reserved = true;
    }
}
