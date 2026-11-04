use crate::card::{can_afford, pay, Card, CardDecks};
use crate::gem::{Gem, GemCounts};
use crate::patron::{meets_patron, random_patrons, Patron};
use crate::player::Player;
use rand::SeedableRng;
use rand::rngs::StdRng;
use thiserror::Error;

pub const WIN_POINTS: u8 = 15;
pub const MAX_TOKENS: u8 = 10;
pub const MAX_RESERVED: usize = 3;
pub const TABLE_SIZE: usize = 3;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    TakeGems(Vec<Gem>),
    BuyTable { tier: u8, slot: usize },
    BuyReserved { idx: usize },
    ReserveTable { tier: u8, slot: usize },
    ReserveDeck { tier: u8 },
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("max tokens")]
    MaxTokens { would: u8 },
    #[error("bank empty")]
    BankEmpty,
    #[error("empty slot")]
    EmptySlot,
    #[error("bad reserve")]
    BadReserveIndex,
    #[error("cannot afford")]
    CannotAfford,
    #[error("reserve full")]
    ReserveFull,
    #[error("deck empty")]
    DeckEmpty,
    #[error("pick gems")]
    PickAtLeastOne,
    #[error("take two or three")]
    TakeTwoOrThree,
    #[error("two same")]
    TwoSameColor,
    #[error("need four in bank")]
    NeedFourInBank,
    #[error("no gold on take")]
    NoGoldOnTake,
    #[error("three differ")]
    ThreeMustDiffer,
    #[error("max three")]
    MaxThreeGems,
}

#[derive(Clone, Debug)]
pub enum LogEntry {
    Took { name: String, gems: String },
    ReservedTable { name: String, tier: u8 },
    ReservedDeck { name: String, tier: u8 },
    Bought {
        name: String,
        tier: u8,
        points: u8,
        bonus: Gem,
    },
    FinalRound { name: String },
    Patron { name: String, points: u8 },
}

#[derive(Clone, Debug)]
pub struct Game {
    pub seed: u64,
    pub bank: GemCounts,
    pub decks: CardDecks,
    pub table: [[Option<Card>; TABLE_SIZE]; 3],
    pub patrons: Vec<Patron>,
    pub players: Vec<Player>,
    pub current: usize,
    pub final_turns_left: Option<u8>,
    pub triggered_by: Option<usize>,
    pub log: Vec<LogEntry>,
}

impl Game {
    pub fn new_human_vs_ai(seed: u64, human_name: &str, cpu_name: &str) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let mut bank = GemCounts::default();
        for g in Gem::BASIC {
            bank.set(g, 4);
        }
        bank.gold = 5;
        let mut decks = CardDecks::new(&mut rng);
        let mut table: [[Option<Card>; TABLE_SIZE]; 3] =
            std::array::from_fn(|_| std::array::from_fn(|_| None));
        for tier in 1..=3 {
            for slot in 0..TABLE_SIZE {
                table[(tier - 1) as usize][slot] = decks.draw(tier);
            }
        }
        let patrons = random_patrons(3, &mut rng);
        let players = vec![
            Player {
                name: human_name.into(),
                is_human: true,
                ..Default::default()
            },
            Player {
                name: cpu_name.into(),
                is_human: false,
                ..Default::default()
            },
        ];
        Self {
            seed,
            bank,
            decks,
            table,
            patrons,
            players,
            current: 0,
            final_turns_left: None,
            triggered_by: None,
            log: Vec::new(),
        }
    }

    pub fn current_player(&self) -> &Player {
        &self.players[self.current]
    }

    pub fn legal_actions(&self) -> Vec<Action> {
        let p = self.current_player();
        let mut out = Vec::new();
        out.extend(take_gem_actions(&self.bank, p.tokens.total()));
        for tier in 1..=3 {
            for slot in 0..TABLE_SIZE {
                if let Some(ref c) = self.table[(tier - 1) as usize][slot] {
                    if can_afford(&p.tokens, &p.bonuses, &c.cost) {
                        out.push(Action::BuyTable { tier, slot });
                    }
                }
                if p.reserved.len() < MAX_RESERVED {
                    out.push(Action::ReserveTable { tier, slot });
                }
            }
            if p.reserved.len() < MAX_RESERVED {
                out.push(Action::ReserveDeck { tier });
            }
        }
        for (idx, c) in p.reserved.iter().enumerate() {
            if can_afford(&p.tokens, &p.bonuses, &c.cost) {
                out.push(Action::BuyReserved { idx });
            }
        }
        out
    }

    pub fn apply(&mut self, action: Action) -> Result<(), GameError> {
        let name = self.players[self.current].name.clone();
        match action {
            Action::TakeGems(gems) => self.take_gems(&name, gems)?,
            Action::BuyTable { tier, slot } => self.buy_table(&name, tier, slot)?,
            Action::BuyReserved { idx } => self.buy_reserved(&name, idx)?,
            Action::ReserveTable { tier, slot } => self.reserve_table(&name, tier, slot)?,
            Action::ReserveDeck { tier } => self.reserve_deck(&name, tier)?,
        }
        self.check_patrons();
        self.advance_turn();
        Ok(())
    }

    /// Whether `gems` is a legal take for the current player.
    pub fn can_take_gems(&self, gems: &[Gem]) -> bool {
        let held = self.current_player().tokens.total();
        take_gems_valid(&self.bank, held, gems).is_ok()
    }

    fn take_gems(&mut self, name: &str, gems: Vec<Gem>) -> Result<(), GameError> {
        let idx = self.current;
        let held = self.players[idx].tokens.total();
        take_gems_valid(&self.bank, held, gems.as_slice())?;
        for g in &gems {
            if self.bank.get(*g) == 0 {
                return Err(GameError::BankEmpty);
            }
            self.bank.sub(*g, 1);
            self.players[idx].tokens.add(*g, 1);
        }
        self.log.push(LogEntry::Took {
            name: name.into(),
            gems: gems
                .iter()
                .map(|g| g.to_string())
                .collect::<Vec<_>>()
                .join(""),
        });
        Ok(())
    }

    fn buy_table(&mut self, name: &str, tier: u8, slot: usize) -> Result<(), GameError> {
        let card = self
            .table
            .get((tier - 1) as usize)
            .and_then(|row| row.get(slot))
            .and_then(|c| c.clone())
            .ok_or(GameError::EmptySlot)?;
        self.purchase_card(name, card)?;
        self.table[(tier - 1) as usize][slot] = self.decks.draw(tier);
        Ok(())
    }

    fn buy_reserved(&mut self, name: &str, idx: usize) -> Result<(), GameError> {
        let idx_player = self.current;
        let card = self.players[idx_player]
            .reserved
            .get(idx)
            .cloned()
            .ok_or(GameError::BadReserveIndex)?;
        if !can_afford(
            &self.players[idx_player].tokens,
            &self.players[idx_player].bonuses,
            &card.cost,
        ) {
            return Err(GameError::CannotAfford);
        }
        self.players[idx_player].reserved.remove(idx);
        self.purchase_card(name, card)
    }

    fn reserve_table(&mut self, name: &str, tier: u8, slot: usize) -> Result<(), GameError> {
        if self.current_player().reserved.len() >= MAX_RESERVED {
            return Err(GameError::ReserveFull);
        }
        let card = self.table[(tier - 1) as usize][slot]
            .take()
            .ok_or(GameError::EmptySlot)?;
        let idx = self.current;
        self.players[idx].reserved.push(card);
        self.grant_gold_to_player(idx);
        self.table[(tier - 1) as usize][slot] = self.decks.draw(tier);
        self.log.push(LogEntry::ReservedTable {
            name: name.into(),
            tier,
        });
        Ok(())
    }

    fn reserve_deck(&mut self, name: &str, tier: u8) -> Result<(), GameError> {
        if self.current_player().reserved.len() >= MAX_RESERVED {
            return Err(GameError::ReserveFull);
        }
        let card = self
            .decks
            .draw(tier)
            .ok_or(GameError::DeckEmpty)?;
        let idx = self.current;
        self.players[idx].reserved.push(card);
        self.grant_gold_to_player(idx);
        self.log.push(LogEntry::ReservedDeck {
            name: name.into(),
            tier,
        });
        Ok(())
    }

    fn grant_gold_to_player(&mut self, idx: usize) {
        if self.bank.gold > 0 && self.players[idx].tokens.total() < MAX_TOKENS {
            self.bank.gold -= 1;
            self.players[idx].tokens.gold += 1;
        }
    }

    fn purchase_card(&mut self, name: &str, card: Card) -> Result<(), GameError> {
        let idx = self.current;
        {
            let p = &mut self.players[idx];
            if !can_afford(&p.tokens, &p.bonuses, &card.cost) {
                return Err(GameError::CannotAfford);
            }
            pay(&mut p.tokens, &p.bonuses, &card.cost, &mut self.bank);
            p.bonuses.add(card.bonus, 1);
            p.points += card.points;
        }
        self.log.push(LogEntry::Bought {
            name: name.into(),
            tier: card.tier,
            points: card.points,
            bonus: card.bonus,
        });
        if self.players[idx].points >= WIN_POINTS && self.triggered_by.is_none() {
            self.triggered_by = Some(idx);
            let n = self.players.len() as u8;
            self.final_turns_left = Some(n.saturating_sub(1));
            self.log.push(LogEntry::FinalRound {
                name: self.players[idx].name.clone(),
            });
        }
        Ok(())
    }

    fn check_patrons(&mut self) {
        let idx = self.current;
        let bonuses = self.players[idx].bonuses.clone();
        let mut taken = Vec::new();
        for (pi, pat) in self.patrons.iter().enumerate() {
            if meets_patron(&bonuses, pat) {
                taken.push(pi);
            }
        }
        if taken.is_empty() {
            return;
        }
        let pi = taken[0];
        let pat = self.patrons.remove(pi);
        self.players[idx].patrons.push(pat.id);
        self.players[idx].points += pat.points;
        self.log.push(LogEntry::Patron {
            name: self.players[idx].name.clone(),
            points: pat.points,
        });
    }

    fn advance_turn(&mut self) {
        self.current = (self.current + 1) % self.players.len();
        if let Some(left) = self.final_turns_left.as_mut() {
            *left = left.saturating_sub(1);
        }
    }

    pub fn is_over(&self) -> bool {
        self.final_turns_left == Some(0)
    }

    pub fn winner(&self) -> Option<usize> {
        if !self.is_over() {
            return None;
        }
        let mut best = 0usize;
        for (i, p) in self.players.iter().enumerate() {
            if p.total_points() > self.players[best].total_points() {
                best = i;
            }
        }
        Some(best)
    }
}

fn take_gems_valid(bank: &GemCounts, held: u8, gems: &[Gem]) -> Result<(), GameError> {
    validate_take(bank, gems)?;
    let new_total = held + gems.len() as u8;
    if new_total > MAX_TOKENS {
        return Err(GameError::MaxTokens { would: new_total });
    }
    Ok(())
}

fn validate_take(bank: &GemCounts, gems: &[Gem]) -> Result<(), GameError> {
    if gems.is_empty() {
        return Err(GameError::PickAtLeastOne);
    }
    if gems.len() == 1 {
        return Err(GameError::TakeTwoOrThree);
    }
    if gems.len() == 2 {
        if gems[0] != gems[1] {
            return Err(GameError::TwoSameColor);
        }
        if bank.get(gems[0]) < 4 {
            return Err(GameError::NeedFourInBank);
        }
        return Ok(());
    }
    if gems.len() == 3 {
        if gems.iter().any(|g| !g.is_basic()) {
            return Err(GameError::NoGoldOnTake);
        }
        for i in 0..gems.len() {
            for j in (i + 1)..gems.len() {
                if gems[i] == gems[j] {
                    return Err(GameError::ThreeMustDiffer);
                }
            }
        }
        return Ok(());
    }
    Err(GameError::MaxThreeGems)
}

fn take_gem_actions(bank: &GemCounts, held: u8) -> Vec<Action> {
    let mut out = Vec::new();
    if held >= MAX_TOKENS {
        return out;
    }
    for &g in &Gem::BASIC {
        if bank.get(g) >= 4 && held + 2 <= MAX_TOKENS {
            out.push(Action::TakeGems(vec![g, g]));
        }
    }
    let avail: Vec<Gem> = Gem::BASIC
        .into_iter()
        .filter(|g| bank.get(*g) > 0)
        .collect();
    if avail.len() >= 3 && held + 3 <= MAX_TOKENS {
        for i in 0..avail.len() {
            for j in (i + 1)..avail.len() {
                for k in (j + 1)..avail.len() {
                    out.push(Action::TakeGems(vec![avail[i], avail[j], avail[k]]));
                }
            }
        }
    }
    out
}
