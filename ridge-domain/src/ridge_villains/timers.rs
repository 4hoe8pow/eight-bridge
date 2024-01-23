use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct VillainsTimer {
    pub timer: Timer,
}
