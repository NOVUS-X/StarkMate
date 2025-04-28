use crate::time_control::{TimeControl, PlayerClock};

pub struct Game {
    // other fields
    pub time_control: TimeControl,
    pub white_clock: PlayerClock,
    pub black_clock: PlayerClock,
}

impl Game {
    pub fn new(time_control: TimeControl) -> Self {
        Self {
            // other fields initialization
            time_control: time_control.clone(),
            white_clock: PlayerClock::new(time_control.initial_time),
            black_clock: PlayerClock::new(time_control.initial_time),
        }
    }

    pub fn handle_move(&mut self, is_white: bool) {
        let clock = if is_white {
            &mut self.white_clock
        } else {
            &mut self.black_clock
        };

        clock.stop();
        clock.apply_increment(self.time_control.increment);
        clock.start();
    }

    pub fn check_time_out(&self) -> Option<&str> {
        if self.white_clock.time_out() {
            Some("Black wins on time")
        } else if self.black_clock.time_out() {
            Some("White wins on time")
        } else {
            None
        }
    }
}