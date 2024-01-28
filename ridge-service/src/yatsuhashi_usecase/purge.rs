use bevy::prelude::*;
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{
    Yatsuhashi, YatsuhashiAddress, YatsuhashiDirection, YatsuhashiTaste,
};

use crate::hero_usecase::create_hero::HeroPositions;

use super::migrate::BondEvent;

pub fn purge_yatsuhashi(
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
            .for_each(|(_, address, _)| {
                // ポジショニング確認
            });
    }
}
