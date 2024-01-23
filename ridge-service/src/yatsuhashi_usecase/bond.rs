use bevy::prelude::*;
use ridge_domain::{
    ridge_hero::hero::Hero,
    ridge_yatsuhashi::yatsuhashi::{
        Yatsuhashi, YatsuhashiAddress, YatsuhashiDirection, YatsuhashiTaste,
    },
};

use crate::hero_usecase::create_hero::HeroPositions;

use super::migrate::BondEvent;

pub fn bond_yatsuhashi(
    mut bond_events: EventReader<BondEvent>,
    mut yatsuhashies: Query<
        (
            &mut YatsuhashiTaste,
            &mut YatsuhashiAddress,
            &mut YatsuhashiDirection,
        ),
        With<Yatsuhashi>,
    >,
    mut hero_positions: ResMut<HeroPositions>,
) {
    for event in bond_events.read() {
        let child_yatsuhashi = &event.child;

        yatsuhashies
            .iter_mut()
            .map(|(taste, address, direction)| (taste, address.clone(), direction))
            .filter(|(_, address, _)| *address == *child_yatsuhashi)
            .for_each(|(_, _, mut direction)| {
                hero_positions.add_hero(Hero {
                    row: child_yatsuhashi.row,
                    col: child_yatsuhashi.col,
                    past_row: child_yatsuhashi.row,
                    past_col: child_yatsuhashi.col,
                });
                *direction = YatsuhashiDirection::default();
            });
    }
}
