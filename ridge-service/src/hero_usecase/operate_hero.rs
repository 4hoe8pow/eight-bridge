use bevy::prelude::*;
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{Yatsuhashi, YatsuhashiAddress, YatsuhashiTaste};

use super::create_hero::HeroPositions;

pub fn operate_hero(
    mut hero_positions: ResMut<HeroPositions>,
    keys: Res<Input<KeyCode>>,
    mut yatsuhashies: Query<(&mut YatsuhashiTaste, &mut YatsuhashiAddress), With<Yatsuhashi>>,
) {
    // 現在のHero の味を消去
    for hero_position in &hero_positions.hero {
        yatsuhashies
            .iter_mut()
            .filter(|(_, address)| {
                address.row == hero_position.row && address.col == hero_position.col
            })
            .for_each(|(mut taste, _)| *taste = YatsuhashiTaste::default());
    }
    if keys.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        for hero_position in &mut hero_positions.hero {
            hero_position.migrate(1, 0);
            if hero_position.row > 6 {
                hero_position.migrate(0, -1);
            } else if hero_position.row < 6 {
                hero_position.migrate(0, 1);
            }
        }
    }
    if keys.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        for hero_position in &mut hero_positions.hero {
            hero_position.migrate(0, -1);
        }
    }
    if keys.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        for hero_position in &mut hero_positions.hero {
            hero_position.migrate(-1, 0);
            if hero_position.row > 5 {
                hero_position.migrate(0, 1);
            } else if hero_position.row < 5 {
                hero_position.migrate(0, -1);
            }
        }
    }
    if keys.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        for hero_position in &mut hero_positions.hero {
            hero_position.migrate(0, 1);
        }
    }

    for (mut taste, address) in yatsuhashies.iter_mut() {
        for hero_position in &hero_positions.hero {
            if address.row == hero_position.row
                && address.col == hero_position.col
                && *taste == YatsuhashiTaste::default()
            {
                *taste = YatsuhashiTaste::Sesami;
            }
        }
    }
}
