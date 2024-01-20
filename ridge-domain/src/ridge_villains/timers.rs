use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct CinnamonTimer {
    pub timer: Timer,
}
