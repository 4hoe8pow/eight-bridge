use bevy::prelude::*;

/// 操作基準
#[derive(Resource)]
pub struct HeroPosition {
    pub row: i8,
    pub col: i8,
    pub past_row: i8,
    pub past_col: i8,
}

impl HeroPosition {
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
}
