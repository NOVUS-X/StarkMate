#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_time_control() {
        let time_control = TimeControl {
            initial_time: Duration::from_secs(300),
            increment: Duration::from_secs(2),
            delay: Duration::from_secs(1),
        };

        let mut clock = PlayerClock::new(time_control.initial_time);
        clock.start();
        std::thread::sleep(Duration::from_secs(1));
        clock.stop();

        assert!(clock.remaining_time <= Duration::from_secs(299));
    }
}
