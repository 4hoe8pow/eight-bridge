use bevy::prelude::*;
use ridge_domain::{
    ridge_hero::hero::Hero,
    ridge_yatsuhashi::yatsuhashi::{
        Yatsuhashi, YatsuhashiAddress, YatsuhashiDirection, YatsuhashiTaste,
    },
};

use crate::hero_usecase::create_hero::HeroPositions;

#[derive(Event)]
pub struct PurgeEvent {
    pub row: i8,
    pub col: i8,
}

pub fn purge_yatsuhashi(
    mut yatsuhashies: Query<
        (
            &mut YatsuhashiTaste,
            &mut YatsuhashiAddress,
            &mut YatsuhashiDirection,
        ),
        With<Yatsuhashi>,
    >,
    hero_positions: ResMut<HeroPositions>,
    mut purge_trigger: EventWriter<PurgeEvent>,
) {
    for (mut taste, address, mut direction) in yatsuhashies.iter_mut() {
        if *taste == YatsuhashiTaste::Sesami {
            let x = Hero {
                row: address.row,
                col: address.col,
                past_row: -1,
                past_col: -1,
            };
            let other_heros = &x.ref_neighbor();
            if x.is_regular() && hero_positions.is_hexgon(other_heros) {
                *taste = YatsuhashiTaste::default();
                *direction = YatsuhashiDirection::default();
            }
        }
    }
}
