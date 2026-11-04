use gemsmith::gem::{Gem, GemCounts};

/// Toggle a basic gem in the take selection (Splendor: 2 same if stack ≥4, or up to 3 different).
pub fn toggle_pick(pick: &mut Vec<Gem>, g: Gem, bank: &GemCounts) {
    if !g.is_basic() {
        return;
    }
    let n = pick.iter().filter(|x| **x == g).count();
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
        let first = pick[0];
        if first == g {
            if bank.get(g) >= 4 {
                pick.push(g);
            }
        } else if bank.get(g) > 0 {
            pick.push(g);
        }
        return;
    }
    if pick[0] != pick[1] && bank.get(g) > 0 && !pick.contains(&g) {
        pick.push(g);
    }
}

pub fn pick_count(pick: &[Gem], g: Gem) -> u8 {
    pick.iter().filter(|x| **x == g).count() as u8
}
