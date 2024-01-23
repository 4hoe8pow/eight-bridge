use std::time::Duration;

use bevy::prelude::*;
use ridge_domain::ridge_villains::timers::VillainsTimer;

pub fn start_timer(mut commands: Commands) {
    commands.insert_resource(VillainsTimer {
        timer: Timer::new(Duration::from_secs(5), TimerMode::Repeating),
    });
}
