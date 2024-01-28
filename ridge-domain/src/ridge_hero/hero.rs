/// 操作基準
#[derive(Default, Debug)]
pub struct Hero {
    pub row: i8,
    pub col: i8,
    pub past_row: i8,
    pub past_col: i8,
}

impl Hero {
    pub fn my_threshold(&self) -> i8 {
        match self.row {
            0 | 11 => 12,
            1 | 10 => 14,
            2 | 9 => 16,
            3 | 8 => 18,
            4 | 7 => 20,
            5 | 6 => 22,
            _ => 0,
        }
    }

    pub fn at_left(&self) -> Option<Hero> {
        match self.col {
            c if c > 0 => Some(Hero {
                row: self.row,
                col: self.row - 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => None,
        }
    }

    pub fn at_bottom_left(&self) -> Option<Hero> {
        match self.row {
            0 => None,
            r if r < 6 => match self.col {
                c if c > 1 => Some(Hero {
                    row: self.row - 1,
                    col: self.col - 2,
                    past_row: self.past_row,
                    past_col: self.past_col,
                }),
                _ => None,
            },
            r if r == 6 => match self.col {
                c if c > 1 => Some(Hero {
                    row: self.row - 1,
                    col: self.col - 1,
                    past_row: self.past_row,
                    past_col: self.past_col,
                }),
                _ => None,
            },
            _ => Some(Hero {
                row: self.row - 1,
                col: self.col,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
        }
    }

    pub fn at_bottom(&self) -> Option<Hero> {
        match self.row {
            0 => None,
            r if r < 6 => match self.col {
                c if c > 0 => Some(Hero {
                    row: self.row - 1,
                    col: self.col - 1,
                    past_row: self.past_row,
                    past_col: self.past_col,
                }),
                _ => None,
            },
            _ => Some(Hero {
                row: self.row - 1,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
        }
    }

    pub fn at_bottom_right(&self) -> Option<Hero> {
        match self.row {
            0 => None,
            r if r < 6 && self.col < self.my_threshold() - 1 => Some(Hero {
                row: self.row - 1,
                col: self.col,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r >= 6 => Some(Hero {
                row: self.row - 1,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => None,
        }
    }

    pub fn at_right(&self) -> Option<Hero> {
        match self.col {
            c if c < self.my_threshold() => Some(Hero {
                row: self.row,
                col: self.row + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => None,
        }
    }

    pub fn at_top_right(&self) -> Option<Hero> {
        match self.row {
            11 => None,
            r if r > 6 && self.col < self.my_threshold() - 1 => Some(Hero {
                row: self.row + 1,
                col: self.col,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r == 6 && self.col < self.my_threshold() => Some(Hero {
                row: self.row + 1,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r < 6 => Some(Hero {
                row: self.row + 1,
                col: self.col + 2,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => None,
        }
    }

    pub fn at_top(&self) -> Option<Hero> {
        match self.row {
            11 => None,
            r if r > 6 && self.col > 0 && self.col < self.my_threshold() => Some(Hero {
                row: self.row + 1,
                col: self.col - 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r == 6 => Some(Hero {
                row: self.row + 1,
                col: self.col,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => Some(Hero {
                row: self.row + 1,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
        }
    }

    pub fn at_top_left(&self) -> Option<Hero> {
        match self.row {
            11 => None,
            r if r < 6 && self.col < self.my_threshold() - 1 => Some(Hero {
                row: self.row - 1,
                col: self.col,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r >= 6 => Some(Hero {
                row: self.row - 1,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => None,
        }
    }

    pub fn migrate(&mut self, drow: i8, dcol: i8) {
        self.past_row = self.row;
        self.past_col = self.col;
        self.row += drow;
        self.col += dcol;
    }

    pub fn revert(&mut self) {
        self.row = self.past_row;
        self.col = self.past_col;
    }

    pub fn is_regular(&self) -> bool {
        match self.row {
            r if r > 5 => self.col % 2 == 0,
            _ => self.col % 2 != 0,
        }
    }

    pub fn ref_neighbor(&self) -> Vec<&Hero> {
        let heros = vec![self];

        match self.is_regular() {
            true => {}
            false => {}
        }

        heros
    }
}

impl PartialEq for Hero {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
            && self.col == other.col
            && self.past_row == other.past_row
            && self.past_col == other.past_col
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_at_right_and_at_left_equality() {
        let hero1 = Hero {
            row: 0,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        let hero2 = Hero {
            row: 0,
            col: 2,
            past_row: 0,
            past_col: 0,
        };

        let result1 = hero1.at_right();
        let result2 = hero2.at_left();

        assert_eq!(result1, result2);
    }

    #[test]
    fn test_my_threshold() {
        let hero = Hero {
            row: 0,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        assert_eq!(hero.my_threshold(), 12);

        let hero = Hero {
            row: 1,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        assert_eq!(hero.my_threshold(), 14);

        // Add more test cases for other row values
    }

    #[test]
    fn test_at_left() {
        let hero = Hero {
            row: 0,
            col: 1,
            past_row: 0,
            past_col: 0,
        };
        assert_eq!(hero.at_left().unwrap().col, 0);

        // Add more test cases for other scenarios
    }

    // Write similar tests for other methods like `at_bottom_left`, `at_bottom`, etc.
    // Ensure to cover various scenarios and edge cases.

    #[test]
    fn test_is_regular() {
        let hero_regular = Hero {
            row: 6,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        assert!(hero_regular.is_regular());

        let hero_irregular = Hero {
            row: 7,
            col: 1,
            past_row: 0,
            past_col: 0,
        };
        assert!(!hero_irregular.is_regular());

        // Add more test cases as needed
    }

    #[test]
    fn test_ref_neighbor() {
        let hero = Hero {
            row: 6,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        let neighbors = hero.ref_neighbor();
        assert_eq!(neighbors.len(), 1);
        assert_eq!(neighbors[0].row, 6);
        assert_eq!(neighbors[0].col, 0);

        // Add more test cases for regular and irregular heroes
    }
}
