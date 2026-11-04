use crate::gem::{Gem, GemCounts};
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Clone, Debug)]
pub struct Card {
    pub id: u32,
    pub tier: u8,
    pub points: u8,
    pub bonus: Gem,
    pub cost: GemCounts,
}

#[derive(Clone, Debug)]
pub struct CardDecks {
    pub tier1: Vec<Card>,
    pub tier2: Vec<Card>,
    pub tier3: Vec<Card>,
}

impl CardDecks {
    pub fn new(rng: &mut impl Rng) -> Self {
        let mut tier1 = build_tier(1, rng);
        let mut tier2 = build_tier(2, rng);
        let mut tier3 = build_tier(3, rng);
        tier1.shuffle(rng);
        tier2.shuffle(rng);
        tier3.shuffle(rng);
        Self {
            tier1,
            tier2,
            tier3,
        }
    }

    pub fn draw(&mut self, tier: u8) -> Option<Card> {
        match tier {
            1 => self.tier1.pop(),
            2 => self.tier2.pop(),
            3 => self.tier3.pop(),
            _ => None,
        }
    }
}

fn build_tier(tier: u8, rng: &mut impl Rng) -> Vec<Card> {
    let n = match tier {
        1 => 24,
        2 => 18,
        _ => 12,
    };
    (0..n)
        .map(|i| {
            let bonus = Gem::BASIC[rng.gen_range(0..5)];
            let points = match tier {
                1 => if i % 4 == 0 { 1 } else { 0 },
                2 => rng.gen_range(1..=3),
                _ => rng.gen_range(3..=5),
            };
            Card {
                id: (tier as u32) * 1000 + i,
                tier,
                points,
                bonus,
                cost: random_cost(tier, rng),
            }
        })
        .collect()
}

fn random_cost(tier: u8, rng: &mut impl Rng) -> GemCounts {
    let mut c = GemCounts::default();
    let slots = match tier {
        1 => rng.gen_range(2..=4),
        2 => rng.gen_range(3..=5),
        _ => rng.gen_range(4..=7),
    };
    for _ in 0..slots {
        let g = Gem::BASIC[rng.gen_range(0..5)];
        let add = match tier {
            1 => 1,
            2 => rng.gen_range(1..=2),
            _ => rng.gen_range(1..=3),
        };
        c.add(g, add);
    }
    c
}

pub fn can_afford(hand: &GemCounts, bonuses: &GemCounts, cost: &GemCounts) -> bool {
    let mut need_gold = 0u8;
    for g in Gem::BASIC {
        let discount = bonuses.get(g);
        let need = cost.get(g).saturating_sub(discount);
        if hand.get(g) + hand.gold < need + need_gold {
            return false;
        }
        if hand.get(g) < need {
            need_gold += need - hand.get(g);
        }
    }
    hand.gold >= need_gold
}

pub fn pay(hand: &mut GemCounts, bonuses: &GemCounts, cost: &GemCounts, bank: &mut GemCounts) {
    let mut gold_spent = 0u8;
    for g in Gem::BASIC {
        let need = cost.get(g).saturating_sub(bonuses.get(g));
        let from_hand = hand.get(g).min(need);
        hand.sub(g, from_hand);
        bank.add(g, from_hand);
        let rest = need - from_hand;
        if rest > 0 {
            hand.sub(Gem::Gold, rest);
            gold_spent += rest;
        }
    }
    let _ = gold_spent;
}
