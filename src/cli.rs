use crate::card::Card;
use crate::locale::{Language, Locale};
use crate::game::{Action, Game, TABLE_SIZE};
use crate::gem::Gem;
use crate::patron::Patron;
use crate::player::Player;
use std::io::{self, Write};

pub fn print_state(game: &Game) {
    println!("\n========== GEMSMITH ==========");
    println!(
        "Bank  R{} S{} E{} D{} X{} G{}",
        game.bank.ruby,
        game.bank.sapphire,
        game.bank.emerald,
        game.bank.diamond,
        game.bank.onyx,
        game.bank.gold
    );
    for tier in 1..=3 {
        print!("T{tier} ");
        for slot in 0..TABLE_SIZE {
            print!("[{slot}] ");
            match &game.table[(tier - 1) as usize][slot] {
                Some(c) => print_card_short(c),
                None => print!("--- "),
            }
        }
        println!();
    }
    println!("Patrons:");
    for (i, p) in game.patrons.iter().enumerate() {
        print!("  [{i}] ");
        print_patron(p);
        println!();
    }
    for p in &game.players {
        print_player(p);
    }
    let loc = Locale::new(Language::En);
    let log_tail = game
        .log
        .last()
        .map(|e| loc.format_log(e))
        .unwrap_or_default();
    println!(
        "Turn: {} | Log: {}",
        game.current_player().name,
        log_tail
    );
}

fn print_card_short(c: &Card) {
    print!("+{} {} cost ", c.points, c.bonus);
    for (g, n) in c.cost.iter_basic() {
        if n > 0 {
            print!("{g}{n}");
        }
    }
    print!(" | ");
}

fn print_patron(p: &Patron) {
    print!("+{} need ", p.points);
    for (g, n) in p.requires.iter_basic() {
        if n > 0 {
            print!("{g}{n}");
        }
    }
}

fn print_player(p: &Player) {
    println!(
        "  {} — {}pt | tok R{} S{} E{} D{} X{} G{} | bonus R{} S{} E{} D{} X{} | res {}",
        p.name,
        p.points,
        p.tokens.ruby,
        p.tokens.sapphire,
        p.tokens.emerald,
        p.tokens.diamond,
        p.tokens.onyx,
        p.tokens.gold,
        p.bonuses.ruby,
        p.bonuses.sapphire,
        p.bonuses.emerald,
        p.bonuses.diamond,
        p.bonuses.onyx,
        p.reserved.len()
    );
}

pub fn prompt_human_action(game: &Game) -> Option<Action> {
    let legal = game.legal_actions();
    if legal.is_empty() {
        println!("No legal moves (should not happen).");
        return None;
    }
    println!("\nActions:");
    println!("  1) Take gems");
    println!("  2) Buy from table");
    println!("  3) Buy reserved");
    println!("  4) Reserve from table");
    println!("  5) Reserve from deck top");
    print!("Choice [1-5]: ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok()?;
    match line.trim() {
        "1" => prompt_take_gems(game),
        "2" => prompt_buy_table(game),
        "3" => prompt_buy_reserved(game),
        "4" => prompt_reserve_table(game),
        "5" => prompt_reserve_deck(game),
        _ => {
            println!("Invalid.");
            None
        }
    }
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    line.trim().to_string()
}

fn prompt_take_gems(game: &Game) -> Option<Action> {
    println!("Take: a) 3 different (enter e.g. RSE)  b) 2 same (e.g. RR, need 4+ in bank)");
    print!("> ");
    let _ = io::stdout().flush();
    let s = read_line().to_uppercase();
    let gems = parse_gem_string(&s)?;
    let action = Action::TakeGems(gems);
    if game.legal_actions().contains(&action) {
        Some(action)
    } else {
        println!("Illegal take.");
        None
    }
}

fn parse_gem_string(s: &str) -> Option<Vec<Gem>> {
    let mut out = Vec::new();
    for ch in s.chars() {
        let g = match ch {
            'R' => Gem::Ruby,
            'S' => Gem::Sapphire,
            'E' => Gem::Emerald,
            'D' => Gem::Diamond,
            'X' => Gem::Onyx,
            _ => return None,
        };
        out.push(g);
    }
    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

fn prompt_buy_table(game: &Game) -> Option<Action> {
    print!("Tier (1-3): ");
    let t: u8 = read_line().parse().ok()?;
    print!("Slot (0-2): ");
    let slot: usize = read_line().parse().ok()?;
    let a = Action::BuyTable { tier: t, slot };
    legal_or_none(game, a)
}

fn prompt_buy_reserved(game: &Game) -> Option<Action> {
    let p = game.current_player();
    for (i, c) in p.reserved.iter().enumerate() {
        print!("[{i}] ");
        print_card_short(c);
        println!();
    }
    print!("Index: ");
    let idx: usize = read_line().parse().ok()?;
    legal_or_none(game, Action::BuyReserved { idx })
}

fn prompt_reserve_table(game: &Game) -> Option<Action> {
    print!("Tier (1-3): ");
    let t: u8 = read_line().parse().ok()?;
    print!("Slot (0-2): ");
    let slot: usize = read_line().parse().ok()?;
    legal_or_none(game, Action::ReserveTable { tier: t, slot })
}

fn prompt_reserve_deck(game: &Game) -> Option<Action> {
    print!("Tier (1-3): ");
    let t: u8 = read_line().parse().ok()?;
    legal_or_none(game, Action::ReserveDeck { tier: t })
}

fn legal_or_none(game: &Game, a: Action) -> Option<Action> {
    if game.legal_actions().contains(&a) {
        Some(a)
    } else {
        println!("Illegal.");
        None
    }
}
