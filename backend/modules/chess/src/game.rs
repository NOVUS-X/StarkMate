use crate::time_control::{TimeControl, PlayerClock};
use crate::db::schema::time_controls; // Assuming a database interaction library is used
use diesel::prelude::*;
use std::time::Duration;

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
        use diesel::dsl::insert_into;
        let white_ms = self.white_clock.get_real_time_remaining().as_millis() as i64;
        let black_ms = self.black_clock.get_real_time_remaining().as_millis() as i64;

        insert_into(time_controls::table)
            .values((
                time_controls::game_id.eq(game_id),
                time_controls::white_remaining_time.eq(white_ms),
                time_controls::black_remaining_time.eq(black_ms),
                time_controls::initial_time.eq(self.time_control.initial_time.as_millis() as i64),
                time_controls::increment.eq(self.time_control.increment.as_millis() as i64),
                time_controls::delay.eq(self.time_control.delay.as_millis() as i64),
            ))
            .on_conflict(time_controls::game_id)
            .do_update()
            .set((
                time_controls::white_remaining_time.eq(white_ms),
                time_controls::black_remaining_time.eq(black_ms),
            ))
            .execute(conn)?;
        Ok(())
    }

    pub fn load_clocks_from_db(&mut self, conn: &PgConnection, game_id: uuid::Uuid) -> QueryResult<()> {
        use time_controls::{white_remaining_time, black_remaining_time};
        let (white_ms, black_ms): (i64, i64) = time_controls::table
            .select((white_remaining_time, black_remaining_time))
            .find(game_id)
            .first(conn)?;

        self.white_clock
            .set_remaining_time(Duration::from_millis(white_ms as u64));
        self.black_clock
            .set_remaining_time(Duration::from_millis(black_ms as u64));
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

        let opponent_clock = if is_white {
            &mut self.black_clock
        } else {
            &mut self.white_clock
        };
        opponent_clock.start();
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