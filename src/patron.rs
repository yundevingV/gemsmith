use crate::gem::{Gem, GemCounts};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Patron {
    pub id: u8,
    pub points: u8,
    pub requires: GemCounts,
}

pub fn random_patrons(n: usize, rng: &mut impl Rng) -> Vec<Patron> {
    (0..n)
        .map(|id| {
            let mut requires = GemCounts::default();
            let mut picked = Gem::BASIC.to_vec();
            picked.shuffle(rng);
            for g in picked.into_iter().take(3) {
                requires.add(g, rng.gen_range(3..=4));
            }
            Patron {
                id: id as u8,
                points: 3,
                requires,
            }
        })
        .collect()
}

pub fn meets_patron(bonuses: &GemCounts, p: &Patron) -> bool {
    Gem::BASIC.iter().all(|&g| bonuses.get(g) >= p.requires.get(g))
}
