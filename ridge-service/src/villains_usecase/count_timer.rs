use std::time::Duration;

use bevy::prelude::*;
use ridge_domain::ridge_villains::timers::CinnamonTimer;

pub fn start_timer(mut commands: Commands) {
    commands.insert_resource(CinnamonTimer {
        timer: Timer::new(Duration::from_secs(3), TimerMode::Repeating),
    });
}
