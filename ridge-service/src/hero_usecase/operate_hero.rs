use bevy::prelude::*;
use ridge_domain::{
    ridge_hero::hero::HeroPosition,
    ridge_yatsuhashi::yatsuhashi::{Yatsuhashi, YatsuhashiAddress, YatsuhashiTaste},
};

pub fn operate_hero(
    mut hero_position: ResMut<HeroPosition>,
    keys: Res<Input<KeyCode>>,
    mut yatsuhashies: Query<(&mut YatsuhashiTaste, &mut YatsuhashiAddress), With<Yatsuhashi>>,
) {
    // 現在のHeroPosition の味を消去
    yatsuhashies
        .iter_mut()
        .filter(|(_, address)| address.row == hero_position.row && address.col == hero_position.col)
        .for_each(|(mut taste, _)| *taste = YatsuhashiTaste::default());

    if keys.any_just_pressed([KeyCode::W, KeyCode::Up]) {
        hero_position.migrate(1, 0);
        if hero_position.row > 6 {
            hero_position.migrate(0, -1);
        } else if hero_position.row < 6 {
            hero_position.migrate(0, 1);
        }
    }
    if keys.any_just_pressed([KeyCode::A, KeyCode::Left]) {
        hero_position.migrate(0, -1);
    }
    if keys.any_just_pressed([KeyCode::S, KeyCode::Down]) {
        hero_position.migrate(-1, 0);
        if hero_position.row > 5 {
            hero_position.migrate(0, 1);
        } else if hero_position.row < 5 {
            hero_position.migrate(0, -1);
        }
    }
    if keys.any_just_pressed([KeyCode::D, KeyCode::Right]) {
        hero_position.migrate(0, 1);
    }

    for (mut taste, address) in yatsuhashies.iter_mut() {
        if address.row == hero_position.row
            && address.col == hero_position.col
            && *taste == YatsuhashiTaste::default()
        {
            *taste = YatsuhashiTaste::Sesami;
        }
    }
}
