use std::collections::HashMap;

use crate::{
    commands::ReserveSeatCommand,
    schedules::Schedules,
    screening::{MovieId, Room, RoomId, Screening, ScreeningSchedule, Seats, StartTime},
};
pub struct ReserveSeatCommandHandler {
    schedules: Schedules,
}

impl ReserveSeatCommandHandler {
    pub fn handle(&mut self, command: &ReserveSeatCommand) -> Result<(), String> {
        let schedule = self.schedules.with(command.screeningId);
        match schedule {
            Some(screening_schedule) => Ok(screening_schedule.book(command.seat.0, command.seat.1)),
            None => Err("No screening scheduled".to_string()),
        }
    }
}

#[test]
fn user_books_a_movie_ticket() {
    let command = ReserveSeatCommand {
        screeningId: 123,
        seat: (1, 1),
    };
    let mut schedules = Schedules::new();
    schedules.add(123, screening_schedule());

    let mut command_handler = ReserveSeatCommandHandler { schedules };
    let command_result = command_handler.handle(&command);

    assert_eq!(Ok(()), command_result)
}

fn screening_schedule() -> ScreeningSchedule {
    ScreeningSchedule {
        id: 123,
        screening: Screening {
            movie: MovieId { id: 12 },
            startTime: StartTime {
                minutes: 0,
                hours: 17,
                day: 15,
                month: 10,
                year: 2021,
            },
        },
        room: Room {
            id: RoomId { id: 1 },
            name: "Green".to_string(),
            seats: Seats {},
        },
    }
}
