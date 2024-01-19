use bevy::prelude::*;
use ridge_domain::ridge_villains::{villain_animation::VillainAnimation, villains::VillainsBundle};

pub fn install_villains(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // read original texture tiles
    let texture_handle: Handle<Image> = asset_server.load("fox.png");
    // tiles -> atlas
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 14, 7, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // 場所を渡して敵スプライト配置
    commands.spawn(create_villain_bundle(
        Vec3::new(200.0, 200.0, 0.),
        texture_atlas_handle,
    ));
}

fn create_villain_bundle(
    translation: Vec3,
    texture_atlas_handle: Handle<TextureAtlas>,
) -> VillainsBundle {
    VillainsBundle {
        sprite: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform {
                translation,
                scale: Vec3::splat(2.0),
                ..default()
            },
            ..default()
        },
        animation: VillainAnimation::default(),
        ..default()
    }
}
