use bevy::{prelude::*, window::PrimaryWindow};
use ridge_domain::ridge_villains::{villain_animation::VillainAnimation, villains::VillainsBundle};

pub fn install_villains(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // read original texture tiles
    let texture_handle: Handle<Image> = asset_server.load("fox.png");
    // tiles -> atlas
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32., 32.), 14, 7, None, None);

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let window = window_query.single();
    let width = window.width() / 2.0;
    let height = window.height() / 2.0;
    // 場所を渡して敵スプライト配置
    commands.spawn(create_villain_bundle(
        Vec3::new(width * 0.9, height * 0.9, 0.0),
        texture_atlas_handle.clone(),
    ));
    commands.spawn(create_villain_bundle(
        Vec3::new(-width * 0.9, height * 0.9, 0.0),
        texture_atlas_handle.clone(),
    ));
    commands.spawn(create_villain_bundle(
        Vec3::new(width * 0.9, -height * 0.9, 0.0),
        texture_atlas_handle.clone(),
    ));
    commands.spawn(create_villain_bundle(
        Vec3::new(-width * 0.9, -height * 0.9, 0.0),
        texture_atlas_handle.clone(),
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
