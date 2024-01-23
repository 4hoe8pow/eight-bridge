use bevy::prelude::*;
use ridge_domain::{
    ridge_villains::timers::VillainsTimer, ridge_yatsuhashi::yatsuhashi::YatsuhashiTaste,
};

#[derive(Event)]
pub struct FireEvent {
    pub taste: YatsuhashiTaste,
}

pub fn yatsuhashi_timer(
    time: Res<Time>,
    mut timer: ResMut<VillainsTimer>,
    mut event: EventWriter<FireEvent>,
) {
    let timer = &mut timer;

    if timer.timer.tick(time.delta()).just_finished() {
        event.send(FireEvent {
            taste: YatsuhashiTaste::random(),
        })
    }
}
