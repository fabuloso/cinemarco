use std::collections::HashMap;

use crate::events::event::Event;

impl ScreeningSchedule {
    pub fn book(&mut self, row: u16, number: u16) -> bool {
        if self.room.is_free(row, number) {
            self.room.reserve_seat(row, number);
            true
        } else {
            false
        }
    }

    pub fn apply(&mut self, event: Event) {
        match event {
            Event::SeatReserved(event) => {
                self.book(event.seat.0, event.seat.1);
            }
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
    pub start_time: StartTime,
}

pub struct StartTime {
    pub minutes: u16,
    pub hours: u16,
    pub day: u16,
    pub month: u8,
    pub year: u16,
}

pub struct MovieId {
    pub id: u32,
}
pub struct Year(u32);

pub struct RoomId {
    pub id: u32,
}

pub struct Room {
    pub id: RoomId,
    pub name: String,
    pub seats: Seats,
}

#[derive(Debug)]
pub struct Seats {
    pub seats: HashMap<String, Seat>,
}

#[derive(Clone, Debug)]
pub struct Seat {
    pub row: u16,
    pub number: u16,
    pub dbox: bool,
    pub reserved: bool,
}

impl Seat {
    fn is_already_reserved(&self) -> bool {
        self.reserved
    }
}

impl Seats {
    fn at(&self, row: u16, number: u16) -> Result<Seat, String> {
        let seat = self.seats.get(&format!("{}_{}", row, number).to_string());
        match seat {
            Some(seat) => Ok(seat.clone()),
            None => Err("Seat is not present".to_string()),
        }
    }

    fn update(&mut self, seat: Seat) {
        self.seats
            .insert(format!("{}_{}", seat.row, seat.number).to_string(), seat);
    }
}

impl Room {
    fn is_free(&self, row: u16, number: u16) -> bool {
        let seat = self.seats.at(row, number);
        !seat.unwrap().is_already_reserved()
    }

    fn reserve_seat(&mut self, row: u16, number: u16) {
        let mut seat = self.seats.at(row, number).unwrap();
        seat.reserved = true;
        self.seats.update(seat);
    }
}
