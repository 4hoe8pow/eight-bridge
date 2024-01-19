use bevy::prelude::*;
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{Yatsuhashi, YatsuhashiTaste};

/// 八つ橋発光
pub fn lightup_yatsuhashi(
    yatsuhashies: Query<(&mut Transform, &mut YatsuhashiTaste), With<Yatsuhashi>>,
) {
    for (transform, taste) in yatsuhashies.iter() {}
}
