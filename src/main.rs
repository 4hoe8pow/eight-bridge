use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use ridge_service::{
    hero_usecase::create_hero::set_hero,
    install_studio,
    villains_usecase::{setup::*, ui::*},
    yatsuhashi_usecase::{factory::*, lightup::lightup_yatsuhashi},
};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    InGame,
}

fn main() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(
            // アプリケーションのウィンドウ設定
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "8 RIDGE".into(),
                    resolution: (720.0, 720.0).into(),
                    resizable: false,
                    present_mode: PresentMode::AutoVsync,
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            }),
        )
        .add_state::<GameState>()
        // 初期設定
        .add_systems(Startup, install_studio)
        // フィールドタイルの設置
        .add_systems(
            OnEnter(GameState::InGame),
            (create_yatsuhashies, install_villains, set_hero),
        )
        .add_systems(
            PostUpdate,
            (start_enemy_animation, lightup_yatsuhashi).run_if(in_state(GameState::InGame)),
        )
        .add_systems(Last, bevy::window::close_on_esc)
        .run();
}
