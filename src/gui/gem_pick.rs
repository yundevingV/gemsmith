use gemsmith::game::{gem_pick_progress_valid, MAX_TOKENS};
use gemsmith::gem::{Gem, GemCounts};
use eframe::egui;

fn simulate_toggle(pick: &mut Vec<Gem>, g: Gem, bank: &GemCounts, held: u8) {
    if !g.is_basic() {
        return;
    }
    let n = pick.iter().filter(|x| **x == g).count();

    // Same color only: tap cycles 0 → 1 → 2 (if bank has 2+) → clear
    if pick.len() <= 2 && pick.iter().all(|x| *x == g) {
        match n {
            0 => {
                if bank.get(g) > 0 {
                    pick.push(g);
                }
            }
            1 => {
                if bank.get(g) >= 2 {
                    let mut two = pick.clone();
                    two.push(g);
                    if gem_pick_progress_valid(bank, held, &two) {
                        pick.push(g);
                    } else {
                        pick.clear();
                    }
                } else {
                    pick.clear();
                }
            }
            _ => pick.clear(),
        }
        return;
    }

    if n > 0 {
        pick.retain(|x| *x != g);
        return;
    }
    if pick.len() == 2 && pick[0] == pick[1] {
        return;
    }
    if pick.len() >= 3 {
        return;
    }
    if pick.is_empty() {
        if bank.get(g) > 0 {
            pick.push(g);
        }
        return;
    }
    if pick.len() == 1 {
        if bank.get(g) > 0 {
            pick.push(g);
        }
        return;
    }
    if pick[0] != pick[1] && bank.get(g) > 0 && !pick.contains(&g) {
        pick.push(g);
    }
}

pub fn can_toggle_pick(pick: &[Gem], g: Gem, bank: &GemCounts, held: u8) -> bool {
    if !g.is_basic() {
        return false;
    }
    if held >= MAX_TOKENS {
        return false;
    }
    let mut trial = pick.to_vec();
    let before = trial.clone();
    simulate_toggle(&mut trial, g, bank, held);
    if trial == before {
        return false;
    }
    if trial.len() < before.len() {
        return true;
    }
    gem_pick_progress_valid(bank, held, &trial)
}

/// Toggle a basic gem in the take selection (1, 2 same, or 3 different).
pub fn toggle_pick(pick: &mut Vec<Gem>, g: Gem, bank: &GemCounts, held: u8) -> bool {
    if !can_toggle_pick(pick, g, bank, held) {
        return false;
    }
    simulate_toggle(pick, g, bank, held);
    true
}

pub fn pick_count(pick: &[Gem], g: Gem) -> u8 {
    pick.iter().filter(|x| **x == g).count() as u8
}

pub fn gem_for_key(key: egui::Key) -> Option<Gem> {
    use eframe::egui::Key;
    use Gem::*;
    match key {
        Key::R => Some(Ruby),
        Key::S => Some(Sapphire),
        Key::E => Some(Emerald),
        Key::D => Some(Diamond),
        Key::X => Some(Onyx),
        _ => None,
    }
}
