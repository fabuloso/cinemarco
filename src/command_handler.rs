use crate::commands::ReserveSeatCommand;
use core::fmt::Error;

pub struct ReserveSeatCommandHandler;

impl ReserveSeatCommandHandler {
    pub fn handle(command: ReserveSeatCommand) -> Result<(), Error> {
        Ok(())
    }
}
