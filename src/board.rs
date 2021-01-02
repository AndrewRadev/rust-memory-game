use ggez::{Context, GameResult};
use std::collections::HashMap;
use crate::cards::Card;

#[derive(Debug)]
pub struct Board {
    row_count: u32,
    col_count: u32,

    width: f32,
    height: f32,

    cards: HashMap<(u32, u32), Card>,
}

impl Board {
    pub fn new(row_count: u32, col_count: u32, width: f32, height: f32) -> Board {
        Board {
            row_count, col_count,
            width, height,
            cards: HashMap::new(),
        }
    }

    pub fn set_card(&mut self, row: u32, col: u32, card: Card) {
        self.cards.insert((row, col), card);
    }

    pub fn interact_with_card(&mut self, x: f32, y: f32) -> Option<&mut Card> {
        let row_size = self.height / (self.row_count as f32);
        let col_size = self.width / (self.col_count as f32);

        self.cards.get_mut(&(
            (y / row_size).floor() as u32,
            (x / col_size).floor() as u32,
        ))
    }

    pub fn update(&mut self, seconds: f32) {
        for card in self.cards.values_mut() {
            card.update(seconds);
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let row_size = self.height / (self.row_count as f32);
        let col_size = self.width / (self.col_count as f32);

        for ((row, col), card) in self.cards.iter() {
            let x = (*col as f32) * col_size + col_size / 2.0;
            let y = (*row as f32) * row_size + row_size / 2.0;

            card.draw(x, y, ctx)?;
        }

        Ok(())
    }
}
