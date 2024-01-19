use bevy::prelude::*;
use ridge_domain::ridge_yatsuhashi::yatsuhashi::{
    YatsuhashiBundle, YatsuhashiStolen, YatsuhashiTaste,
};

/// 若干のmerginをとりつつ正三角形を密に配置
pub fn create_yatsuhashies(mut commands: Commands) {
    commands.spawn(YatsuhashiBundle {
        stolen: YatsuhashiStolen(5),
        taste: YatsuhashiTaste::default(),
        ..default()
    });
}
