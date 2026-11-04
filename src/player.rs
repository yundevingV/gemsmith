use crate::card::Card;
use crate::gem::GemCounts;

#[derive(Clone, Debug, Default)]
pub struct Player {
    pub name: String,
    pub is_human: bool,
    pub tokens: GemCounts,
    pub bonuses: GemCounts,
    pub reserved: Vec<Card>,
    pub patrons: Vec<u8>,
    pub points: u8,
}

impl Player {
    pub fn total_points(&self) -> u8 {
        self.points
    }
}
