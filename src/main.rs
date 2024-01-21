use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use ridge_service::{
    hero_usecase::{create_hero::set_hero, operate_hero::operate_hero},
    install_studio,
    villains_usecase::{
        count_timer::start_timer,
        fire_yatsuhashi::{cinnamon_timer_finished, FireEvent},
        setup_villains::*,
    },
    yatsuhashi_usecase::{
        factory::*,
        lightup::lightup_yatsuhashi,
        migrate::{pop_yatsuhashi, push_yatsuhashi, YatsuhashiQueueEvent},
    },
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
        .add_event::<FireEvent>()
        .add_event::<YatsuhashiQueueEvent>()
        // 初期設定
        .add_systems(Startup, install_studio)
        // フィールドタイルの設置
        .add_systems(
            OnEnter(GameState::InGame),
            (create_yatsuhashies, install_villains, set_hero, start_timer),
        )
        .add_systems(
            Update,
            (operate_hero, cinnamon_timer_finished, fire, push_yatsuhashi)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            PostUpdate,
            (start_enemy_animation, lightup_yatsuhashi, pop_yatsuhashi)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Last, bevy::window::close_on_esc)
        .run();
}
