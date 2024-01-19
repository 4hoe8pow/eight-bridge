use bevy::prelude::*;
use ridge_domain::ridge_villains::{villain_animation::VillainAnimation, villains::Villains};

pub fn start_enemy_animation(
    time: Res<Time>,
    mut villains: Query<(&mut TextureAtlasSprite, &mut VillainAnimation), With<Villains>>,
) {
    let delta = time.delta();
    for (mut sprite, mut anim) in villains.iter_mut() {
        anim.idle.play(sprite.as_mut(), delta);
    }
}
