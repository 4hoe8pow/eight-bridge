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
                col: self.col - 1,
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
            6 => match self.col {
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
            r if r < 6 && self.col > 0 && self.col < self.my_threshold() => Some(Hero {
                row: self.row - 1,
                col: self.col - 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            6 => Some(Hero {
                row: self.row - 1,
                col: self.col,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
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
            r if r == 6 && self.col < self.my_threshold() => Some(Hero {
                row: self.row - 1,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r > 6 => Some(Hero {
                row: self.row - 1,
                col: self.col + 2,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => None,
        }
    }

    pub fn at_right(&self) -> Option<Hero> {
        match self.col {
            c if c < self.my_threshold() - 1 => Some(Hero {
                row: self.row,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            _ => None,
        }
    }

    pub fn at_top_right(&self) -> Option<Hero> {
        match self.row {
            11 => None,
            r if r > 5 && self.col < self.my_threshold() - 1 => Some(Hero {
                row: self.row + 1,
                col: self.col,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r == 5 && self.col < self.my_threshold() => Some(Hero {
                row: self.row + 1,
                col: self.col + 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r < 5 => Some(Hero {
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
            r if r > 5 && self.col > 0 && self.col < self.my_threshold() => Some(Hero {
                row: self.row + 1,
                col: self.col - 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            5 => Some(Hero {
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
            r if r > 5 && self.col > 1 => Some(Hero {
                row: self.row + 1,
                col: self.col - 2,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r == 5 && self.col > 0 => Some(Hero {
                row: self.row + 1,
                col: self.col - 1,
                past_row: self.past_row,
                past_col: self.past_col,
            }),
            r if r < 5 => Some(Hero {
                row: self.row + 1,
                col: self.col,
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

    pub fn ref_neighbor(&self) -> Option<[[Option<Hero>; 5]; 3]> {
        let heros_top = [
            self.at_right(),
            self.at_top_right(),
            self.at_top(),
            self.at_top_left(),
            self.at_left(),
        ];

        let heros_bottom_left = [
            self.at_left(),
            self.at_left().unwrap().at_left(),
            self.at_bottom(),
            self.at_bottom_left(),
            self.at_bottom_left().unwrap().at_left(),
        ];

        let heros_bottom_right = [
            self.at_right(),
            self.at_right().unwrap().at_right(),
            self.at_bottom(),
            self.at_bottom_right(),
            self.at_bottom_right().unwrap().at_right(),
        ];

        match self.is_regular() {
            true => Some([heros_top, heros_bottom_left, heros_bottom_right]),
            false => Some([heros_top, heros_bottom_left, heros_bottom_right]),
        }
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
    }

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
    }

    #[test]
    fn test_middle_offset() {
        let hero1 = Hero {
            row: 6,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        let hero2 = Hero {
            row: 6,
            col: 1,
            past_row: 0,
            past_col: 0,
        };
        assert_eq!(hero1.at_right(), Some(hero2));
        let hero3 = Hero {
            row: 6,
            col: 2,
            past_row: 0,
            past_col: 0,
        };

        let hero4 = Hero {
            row: 5,
            col: 0,
            past_row: 0,
            past_col: 0,
        };

        assert_eq!(hero1.at_bottom(), Some(hero4));
        let hero5 = Hero {
            row: 5,
            col: 1,
            past_row: 0,
            past_col: 0,
        };

        assert_eq!(hero5.at_top_right(), Some(hero3));
        let hero6 = Hero {
            row: 5,
            col: 2,
            past_row: 0,
            past_col: 0,
        };

        let hero7 = Hero {
            row: 7,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        assert_eq!(hero7.at_top_left(), None);
        let hero8 = Hero {
            row: 7,
            col: 1,
            past_row: 0,
            past_col: 0,
        };
        assert_eq!(hero6.at_top_left(), hero8.at_bottom_left());
    }

    #[test]
    fn test_bottom_offset() {
        let hero1 = Hero {
            row: 0,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        let hero2 = Hero {
            row: 0,
            col: 1,
            past_row: 0,
            past_col: 0,
        };
        let hero3 = Hero {
            row: 0,
            col: 2,
            past_row: 0,
            past_col: 0,
        };
        let hero4 = Hero {
            row: 1,
            col: 0,
            past_row: 0,
            past_col: 0,
        };
        let hero5 = Hero {
            row: 1,
            col: 1,
            past_row: 0,
            past_col: 0,
        };
        let hero6 = Hero {
            row: 2,
            col: 2,
            past_row: 0,
            past_col: 0,
        };
        let hero7 = Hero {
            row: 2,
            col: 5,
            past_row: 0,
            past_col: 0,
        };
        let hero8 = Hero {
            row: 2,
            col: 2,
            past_row: 0,
            past_col: 0,
        };
        let hero9 = Hero {
            row: 2,
            col: 4,
            past_row: 0,
            past_col: 0,
        };
        let hero10 = Hero {
            row: 0,
            col: 11,
            past_row: 0,
            past_col: 0,
        };
        let hero11 = Hero {
            row: 1,
            col: 13,
            past_row: 0,
            past_col: 0,
        };
        let hero12 = Hero {
            row: 2,
            col: 15,
            past_row: 0,
            past_col: 0,
        };
        assert_eq!(hero1.at_bottom(), None);
        assert_eq!(hero4.at_left(), None);
        assert_eq!(hero5.at_top(), Some(hero6));
        assert_eq!(hero3.at_top_left(), hero8.at_bottom_right());
        assert_eq!(hero7.at_bottom_left(), hero2.at_top_right());
        assert_eq!(hero9.at_left(), hero8.at_right());
        assert_eq!(hero11.at_right(), None);
        assert_eq!(hero11.at_bottom_right(), None);
        assert_eq!(hero10.at_top_right(), hero12.at_bottom_left());
    }
}
