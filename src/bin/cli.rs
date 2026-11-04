use gemsmith::ai::{pick_action, Difficulty};
use gemsmith::cli::{print_state, prompt_human_action};
use gemsmith::game::Game;
use rand::SeedableRng;
use std::io::{self, Write};

fn main() {
    println!("GEMSMITH — forge gems into prestige (Splendor-style, vs CPU)");
    let diff = prompt_difficulty();
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(42);
    let mut game = Game::new_human_vs_ai(seed, "You", "CPU");
    print_state(&game);

    while !game.is_over() {
        if game.current_player().is_human {
            loop {
                print_state(&game);
                let Some(action) = prompt_human_action(&game) else {
                    continue;
                };
                if let Err(e) = game.apply(action) {
                    println!("Error: {e}");
                    continue;
                }
                break;
            }
        } else {
            print_state(&game);
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed + game.log.len() as u64);
            let action = pick_action(&game, diff, &mut rng);
            println!("CPU: {}", action_label(&action));
            if let Err(e) = game.apply(action) {
                println!("CPU error: {e}");
                if let Some(a) = game.legal_actions().into_iter().next() {
                    let _ = game.apply(a);
                }
            }
        }
    }
    print_state(&game);
    if let Some(w) = game.winner() {
        println!("\nWinner: {} ({} pts)", game.players[w].name, game.players[w].points);
    }
}

fn prompt_difficulty() -> Difficulty {
    println!("Difficulty: 1=Easy  2=Medium  3=Hard");
    print!("> ");
    let _ = io::stdout().flush();
    let mut line = String::new();
    io::stdin().read_line(&mut line).ok();
    match line.trim() {
        "3" => Difficulty::Hard,
        "2" => Difficulty::Medium,
        _ => Difficulty::Easy,
    }
}

fn action_label(a: &gemsmith::game::Action) -> String {
    use gemsmith::game::Action;
    match a {
        Action::TakeGems(g) => format!(
            "take {}",
            g.iter().map(|x| x.to_string()).collect::<String>()
        ),
        Action::BuyTable { tier, slot } => format!("buy T{tier}[{slot}]"),
        Action::BuyReserved { idx } => format!("buy reserved [{idx}]"),
        Action::ReserveTable { tier, slot } => format!("reserve T{tier}[{slot}]"),
        Action::ReserveDeck { tier } => format!("reserve deck T{tier}"),
    }
}
