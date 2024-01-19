use bevy::prelude::*;
use std::time::Duration;
/// 敵のアニメーション
#[derive(Component, Debug)]
pub struct VillainAnimation {
    pub idle: SpriteAnimation,
    pub attack: SpriteAnimation,
}

impl Default for VillainAnimation {
    fn default() -> Self {
        Self {
            idle: SpriteAnimation {
                index: SpriteAnimationIndex { first: 0, last: 4 },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
                ..default()
            },
            attack: SpriteAnimation {
                index: SpriteAnimationIndex {
                    first: 30,
                    last: 43,
                },
                timer: SpriteAnimationTimer(Timer::from_seconds(0.12, TimerMode::Repeating)),
                ..default()
            },
        }
    }
}

/// スプライトアニメーション
#[derive(Component, Default, Debug)]
pub struct SpriteAnimation {
    pub is_playing: bool,
    pub index: SpriteAnimationIndex,
    pub timer: SpriteAnimationTimer,
}

// スプライトアニメーションの構造体にstopとplayの機能を追加
impl SpriteAnimation {
    /// スプライトアニメーションの停止
    /// 次のアニメーションを再生する際は再生している可能性があるものを停止する。
    pub fn stop(&mut self) {
        self.is_playing = false;
    }
    /// スプライトアニメーションの再生
    pub fn play(&mut self, sprite: &mut TextureAtlasSprite, delta: Duration) {
        // 停止中だった場合に再生中をfalseとする。
        if !self.is_playing {
            sprite.index = self.index.first;
            self.is_playing = true;
        }
        // アニメタイマーをtime.delta()秒進める。
        // Timer::from_seconds(0.1, TimerMode::Repeating)と設定しているため、0.1秒経つとjust_finished()がtrueになる。
        self.timer.tick(delta);
        if self.timer.just_finished() {
            sprite.index = if sprite.index == self.index.last {
                self.index.first
            } else {
                sprite.index + 1
            };
        }
    }
}

/// スプライトアニメーションのインデックス
#[derive(Component, Default, Debug)]
pub struct SpriteAnimationIndex {
    pub first: usize,
    pub last: usize,
}

/// スプライトアニメーションのタイマー
#[derive(Component, Default, Debug, Deref, DerefMut)]
pub struct SpriteAnimationTimer(pub Timer);
