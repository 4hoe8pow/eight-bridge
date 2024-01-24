use std::time::Duration;

use bevy::{
    asset::AssetMetaCheck,
    prelude::*,
    time::common_conditions::on_timer,
    window::{PresentMode, WindowTheme},
};
use ridge_service::{
    hero_usecase::{
        create_hero::{init_hero, HeroPositions},
        operate_hero::operate_hero,
    },
    install_studio,
    villains_usecase::{
        count_timer::start_timer,
        fire_yatsuhashi::{yatsuhashi_timer, FireEvent},
        setup_villains::*,
    },
    yatsuhashi_usecase::{
        bond::bond_yatsuhashi,
        factory::*,
        lightup::lightup_yatsuhashi,
        migrate::{pop_yatsuhashi, push_yatsuhashi, BondEvent, ReloadEvent},
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
        .add_event::<ReloadEvent>()
        .add_event::<BondEvent>()
        .add_systems(Startup, install_studio)
        // フィールドタイルの設置
        .init_resource::<HeroPositions>()
        .add_systems(
            OnEnter(GameState::InGame),
            (
                create_yatsuhashies,
                install_villains,
                init_hero,
                start_timer,
            ),
        )
        .add_systems(
            Update,
            (operate_hero, yatsuhashi_timer, fire, push_yatsuhashi)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            pop_yatsuhashi
                .run_if(on_timer(Duration::from_secs_f32(0.1)))
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            PostUpdate,
            (start_enemy_animation, bond_yatsuhashi, lightup_yatsuhashi)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(Last, bevy::window::close_on_esc)
        .run();
}
