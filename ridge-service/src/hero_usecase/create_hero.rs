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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hexgon() {
        // Prepare some heroes
        let hero1 = Hero { row: 0, col: 0, past_row: 0, past_col: 0 };
        let hero2 = Hero { row: 1, col: 0, past_row: 1, past_col: 0 };
        let hero3 = Hero { row: 1, col: 1, past_row: 1, past_col: 1 };
        let hero4 = Hero { row: 0, col: 1, past_row: 0, past_col: 1 };
        let hero5 = Hero { row: -1, col: 1, past_row: -1, past_col: 1 };
        let hero6 = Hero { row: -1, col: 0, past_row: -1, past_col: 0 };
        let hero7 = Hero { row: 3, col: 3, past_row: -1, past_col: 0 };

        // Create HeroPositions instance
        let hero_positions = HeroPositions {
            hero:vec![hero1, hero2, hero3, hero4, hero5, hero6],
        };
        // Test with hexagon heroes
        let hexagon_heroes = vec![hero2, hero3, hero4, hero5, hero6];
        assert!(hero_positions.is_hexgon(&hexagon_heroes));

        // Test with non-hexagon heroes
        let non_hexagon_heroes = vec![hero7, hero2, hero3, hero4, hero5];
        assert!(!hero_positions.is_hexgon(&non_hexagon_heroes));
    }
}
