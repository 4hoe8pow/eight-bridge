use bevy::prelude::*;
use ridge_domain::ridge_hero::hero::Hero;

#[derive(Resource, Default, Debug)]
pub struct HeroPositions {
    pub hero: Vec<Hero>,
}

impl HeroPositions {
    pub fn add_hero(&mut self, new_hero: Hero) {
        self.hero.push(new_hero);
    }

    pub fn is_hexgon(&self, other_heros: &Vec<Hero>) -> bool {
        for other_hero in other_heros {
            debug!("{:?}", other_hero);
            debug!("{:?}", self.hero.contains(other_hero));

            if !self.hero.contains(other_hero) {
                return false;
            }
        }
        true
    }
}

/// ヒーローポジショニング
pub fn init_hero(mut hero_positions: ResMut<HeroPositions>) {
    hero_positions.add_hero(Hero {
        row: 0,
        col: 6,
        past_row: -1,
        past_col: -1,
    });
}
