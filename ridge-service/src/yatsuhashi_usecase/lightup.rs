use bevy::prelude::*;
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{Yatsuhashi, YatsuhashiTaste};

/// 八つ橋発光
pub fn lightup_yatsuhashi(
    yatsuhashies: Query<(&Handle<ColorMaterial>, &mut YatsuhashiTaste), With<Yatsuhashi>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (yatsuhashi_handle, taste) in yatsuhashies.iter() {
        let material = materials.get_mut(yatsuhashi_handle).unwrap();

        match taste {
            YatsuhashiTaste::Tasteless => material.color = Color::rgb_u8(44, 51, 62),
            YatsuhashiTaste::Sesami => material.color = Color::GOLD,
            YatsuhashiTaste::Cinnamon => material.color = Color::rgb_u8(255, 190, 152),
            YatsuhashiTaste::Matcha => material.color = Color::rgb_u8(0, 161, 112),
            YatsuhashiTaste::Ramune => material.color = Color::rgb_u8(158, 180, 211),
            YatsuhashiTaste::Strawberry => material.color = Color::rgb_u8(249, 115, 114),
        }
    }
}
