use crate::{commands::ReserveSeatCommand, schedules::Schedules};
pub struct ReserveSeatCommandHandler {
    schedules: Schedules,
}

impl ReserveSeatCommandHandler {
    pub fn handle(&mut self, command: &ReserveSeatCommand) -> Result<(), String> {
        let schedule = self.schedules.with(command.screening_id);
        match schedule {
            Some(screening) => {
                let result = screening.book(command.seat.0, command.seat.1);
                if result {
                    Ok(())
                } else {
                    Err("Seat already taken, try again".to_string())
                }
            }
            None => todo!(),
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::screening::{
        MovieId, Room, RoomId, Screening, ScreeningSchedule, Seat, Seats, StartTime,
    };

    use super::*;
    #[test]
    fn user_books_a_movie_ticket() {
        let command = ReserveSeatCommand {
            screening_id: 123,
            seat: (1, 1),
        };
        let mut schedules = Schedules::new();
        schedules.add(123, screening_schedule());

        let mut command_handler = ReserveSeatCommandHandler { schedules };
        let command_result = command_handler.handle(&command);

        assert_eq!(Ok(()), command_result)
    }

    #[test]
    fn user_cannot_book_an_already_taken_seat() {
        let mut schedules = Schedules::new();
        schedules.add(123, screening_schedule());

        let mut command_handler = ReserveSeatCommandHandler { schedules };
        let command = ReserveSeatCommand {
            screening_id: 123,
            seat: (1, 1),
        };

        let _ = command_handler.handle(&command);
        let command_result = command_handler.handle(&command);

        assert_eq!(
            Err("Seat already taken, try again".to_string()),
            command_result
        )
    }

    #[test]
    fn user_reservers_two_different_tickets() {
        let mut schedules = Schedules::new();
        schedules.add(123, screening_schedule());

        let mut command_handler = ReserveSeatCommandHandler { schedules };
        let command = ReserveSeatCommand {
            screening_id: 123,
            seat: (0, 0),
        };
        let first_seat_reservation_result = command_handler.handle(&command);

        let command = ReserveSeatCommand {
            screening_id: 123,
            seat: (0, 1),
        };

        let second_seat_reservation_result = command_handler.handle(&command);

        assert!(first_seat_reservation_result.is_ok());
        assert!(second_seat_reservation_result.is_ok());
    }

    fn screening_schedule() -> ScreeningSchedule {
        ScreeningSchedule {
            id: 123,
            screening: Screening {
                movie: MovieId { id: 12 },
                start_time: StartTime {
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
                seats: Seats {
                    seats: [
                        (
                            "0_0".to_string(),
                            Seat {
                                row: 0,
                                number: 0,
                                dbox: false,
                                reserved: false,
                            },
                        ),
                        (
                            "0_1".to_string(),
                            Seat {
                                row: 0,
                                number: 1,
                                dbox: false,
                                reserved: false,
                            },
                        ),
                        (
                            "1_0".to_string(),
                            Seat {
                                row: 1,
                                number: 0,
                                dbox: false,
                                reserved: false,
                            },
                        ),
                        (
                            "1_1".to_string(),
                            Seat {
                                row: 1,
                                number: 1,
                                dbox: false,
                                reserved: false,
                            },
                        ),
                    ]
                    .iter()
                    .cloned()
                    .collect(),
                },
            },
        }
    }
}
