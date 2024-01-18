use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use ridge_service::install_studio;

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
        .add_systems(Startup, install_studio)
        .add_systems(Last, bevy::window::close_on_esc)
        .run();
}
