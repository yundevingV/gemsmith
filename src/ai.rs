use crate::card::can_afford;
use crate::game::{Action, Game};
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

pub fn pick_action(game: &Game, diff: Difficulty, rng: &mut impl Rng) -> Action {
    let actions = game.legal_actions();
    if actions.is_empty() {
        return Action::TakeGems(vec![first_three(game)]);
    }
    match diff {
        Difficulty::Easy => actions[rng.gen_range(0..actions.len())].clone(),
        Difficulty::Medium => {
            let mut best = actions[0].clone();
            let mut best_score = i32::MIN;
            for a in &actions {
                let s = score_action(game, a);
                if s > best_score {
                    best_score = s;
                    best = a.clone();
                }
            }
            best
        }
        Difficulty::Hard => {
            let mut best = actions[0].clone();
            let mut best_score = i32::MIN;
            for a in &actions {
                let s = score_action(game, a) * 2 + score_lookahead(game, a);
                if s > best_score {
                    best_score = s;
                    best = a.clone();
                }
            }
            best
        }
    }
}

fn first_three(game: &Game) -> crate::gem::Gem {
    use crate::gem::Gem;
    for g in Gem::BASIC {
        if game.bank.get(g) > 0 {
            return g;
        }
    }
    Gem::Ruby
}

fn score_action(game: &Game, action: &Action) -> i32 {
    let p = game.current_player();
    match action {
        Action::BuyTable { tier, slot } => {
            if let Some(c) = &game.table[(*tier - 1) as usize][*slot] {
                let mut s = c.points as i32 * 10;
                s += (*tier as i32).min(2);
                s += if c.bonus == most_needed(p) { 4 } else { 0 };
                return s;
            }
            0
        }
        Action::ReserveTable { .. } => 1,
        Action::BuyReserved { idx } => {
            if let Some(c) = p.reserved.get(*idx) {
                return c.points as i32 * 10 + 3;
            }
            0
        }
        Action::ReserveDeck { .. } => 2,
        Action::TakeGems(g) => {
            if g.len() == 2 {
                5
            } else {
                3
            }
        }
    }
}

fn score_lookahead(game: &Game, action: &Action) -> i32 {
    let p = game.current_player();
    match action {
        Action::BuyTable { tier, slot } => {
            if let Some(c) = &game.table[(*tier - 1) as usize][*slot] {
                let mut copy_tokens = p.tokens.clone();
                let mut copy_bonuses = p.bonuses.clone();
                let mut bank = game.bank.clone();
                crate::card::pay(&mut copy_tokens, &p.bonuses, &c.cost, &mut bank);
                copy_bonuses.add(c.bonus, 1);
                return patron_progress(&copy_bonuses, &game.patrons);
            }
            0
        }
        Action::TakeGems(gems) => {
            let mut t = p.tokens.clone();
            for g in gems {
                t.add(*g, 1);
            }
            score_near_buy(&t, &p.bonuses, game)
        }
        _ => 0,
    }
}

fn most_needed(p: &crate::player::Player) -> crate::gem::Gem {
    use crate::gem::Gem;
    Gem::BASIC
        .into_iter()
        .max_by_key(|g| p.bonuses.get(*g))
        .unwrap_or(Gem::Ruby)
}

fn patron_progress(bonuses: &crate::gem::GemCounts, patrons: &[crate::patron::Patron]) -> i32 {
    patrons
        .iter()
        .map(|pat| {
            Gem::BASIC
                .iter()
                .map(|&g| {
                    let need = pat.requires.get(g);
                    let have = bonuses.get(g);
                    have.min(need) as i32
                })
                .sum::<i32>()
        })
        .max()
        .unwrap_or(0)
}

use crate::gem::Gem;

fn score_near_buy(
    tokens: &crate::gem::GemCounts,
    bonuses: &crate::gem::GemCounts,
    game: &Game,
) -> i32 {
    let mut score = 0;
    for tier in 1..=3 {
        for slot in 0..crate::game::TABLE_SIZE {
            if let Some(c) = &game.table[(tier - 1) as usize][slot] {
                if can_afford(tokens, bonuses, &c.cost) {
                    score += 8;
                } else {
                    score += 1;
                }
            }
        }
    }
    score
}
