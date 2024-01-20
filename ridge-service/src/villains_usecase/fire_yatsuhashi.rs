use bevy::prelude::*;
use ridge_domain::{
    ridge_villains::timers::CinnamonTimer, ridge_yatsuhashi::yatsuhashi::YatsuhashiTaste,
};

#[derive(Event)]
pub struct FireEvent {
    pub taste: YatsuhashiTaste,
}

pub fn cinnamon_timer_finished(
    time: Res<Time>,
    mut timer: ResMut<CinnamonTimer>,
    mut event: EventWriter<FireEvent>,
) {
    let timer = &mut timer;

    if timer.timer.tick(time.delta()).just_finished() {
        event.send(FireEvent {
            taste: YatsuhashiTaste::Cinnamon,
        })
    }
}
