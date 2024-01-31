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
    pub hexagon: Vec<Hero>,
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
    mut hero_positions: ResMut<HeroPositions>,
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
            let other_heros: &Vec<Hero> = &x.ref_neighbor();

            if x.is_regular()
                && x.col < x.my_threshold() - 1
                && hero_positions.is_hexgon(other_heros)
            {
                *taste = YatsuhashiTaste::default();
                *direction = YatsuhashiDirection::default();
                // other_heros をすべてhero_positionsから削除
                other_heros
                    .iter()
                    .for_each(|hero| hero_positions.remove_hero(hero));

                purge_trigger.send(PurgeEvent {
                    hexagon: other_heros.to_vec(),
                });
            }
        }
    }
}

pub fn scoring(
    mut yatsuhashies: Query<
        (
            &mut YatsuhashiTaste,
            &mut YatsuhashiAddress,
            &mut YatsuhashiDirection,
        ),
        With<Yatsuhashi>,
    >,
    mut purge_trigger: EventReader<PurgeEvent>,
) {
    for event in purge_trigger.read() {
        let hexgon = &event.hexagon;
        eprintln!("{:?}", hexgon);
    }
}
