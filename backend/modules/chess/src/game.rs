use crate::time_control::{TimeControl, PlayerClock};
use crate::db::schema::time_controls; // Assuming a database interaction library is used
use diesel::prelude::*;

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

    pub fn save_clocks_to_db(&self, conn: &PgConnection, game_id: uuid::Uuid) -> QueryResult<()> {
        diesel::update(time_controls::table.find(game_id))
            .set((
                time_controls::white_remaining_time.eq(self.white_clock.get_real_time_remaining().as_secs() as i64),
                time_controls::black_remaining_time.eq(self.black_clock.get_real_time_remaining().as_secs() as i64),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn load_clocks_from_db(&mut self, conn: &PgConnection, game_id: uuid::Uuid) -> QueryResult<()> {
        let record = time_controls::table.find(game_id).first::<(i64, i64)>(conn)?;
        self.white_clock.set_remaining_time(Duration::from_secs(record.0 as u64));
        self.black_clock.set_remaining_time(Duration::from_secs(record.1 as u64));
        Ok(())
    }

    pub fn handle_move(&mut self, is_white: bool) {
        let clock = if is_white {
            &mut self.white_clock
        } else {
            &mut self.black_clock
        };

        clock.stop();
        clock.apply_delay(self.time_control.delay);
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