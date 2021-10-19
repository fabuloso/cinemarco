pub enum Event {
    SeatReserved(SeatReserved),
}

pub struct SeatReserved {
    pub screening_id: u32,
    pub seat: (u16, u16),
}
