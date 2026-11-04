use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Gem {
    Ruby,
    Sapphire,
    Emerald,
    Diamond,
    Onyx,
    Gold,
}

impl Gem {
    pub const BASIC: [Gem; 5] = [
        Gem::Ruby,
        Gem::Sapphire,
        Gem::Emerald,
        Gem::Diamond,
        Gem::Onyx,
    ];

    pub fn is_basic(self) -> bool {
        !matches!(self, Gem::Gold)
    }

    pub fn label(self) -> &'static str {
        match self {
            Gem::Ruby => "R",
            Gem::Sapphire => "S",
            Gem::Emerald => "E",
            Gem::Diamond => "D",
            Gem::Onyx => "X",
            Gem::Gold => "G",
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Gem::Ruby => "ruby",
            Gem::Sapphire => "sapphire",
            Gem::Emerald => "emerald",
            Gem::Diamond => "diamond",
            Gem::Onyx => "onyx",
            Gem::Gold => "gold",
        }
    }
}

impl fmt::Display for Gem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

#[derive(Clone, Debug, Default, Copy)]
pub struct GemCounts {
    pub ruby: u8,
    pub sapphire: u8,
    pub emerald: u8,
    pub diamond: u8,
    pub onyx: u8,
    pub gold: u8,
}

impl GemCounts {
    pub fn get(&self, g: Gem) -> u8 {
        match g {
            Gem::Ruby => self.ruby,
            Gem::Sapphire => self.sapphire,
            Gem::Emerald => self.emerald,
            Gem::Diamond => self.diamond,
            Gem::Onyx => self.onyx,
            Gem::Gold => self.gold,
        }
    }

    pub fn set(&mut self, g: Gem, v: u8) {
        match g {
            Gem::Ruby => self.ruby = v,
            Gem::Sapphire => self.sapphire = v,
            Gem::Emerald => self.emerald = v,
            Gem::Diamond => self.diamond = v,
            Gem::Onyx => self.onyx = v,
            Gem::Gold => self.gold = v,
        }
    }

    pub fn add(&mut self, g: Gem, n: u8) {
        self.set(g, self.get(g).saturating_add(n));
    }

    pub fn sub(&mut self, g: Gem, n: u8) {
        self.set(g, self.get(g).saturating_sub(n));
    }

    pub fn total(&self) -> u8 {
        self.ruby
            + self.sapphire
            + self.emerald
            + self.diamond
            + self.onyx
            + self.gold
    }

    pub fn iter_basic(&self) -> impl Iterator<Item = (Gem, u8)> + use<'_> {
        Gem::BASIC
            .into_iter()
            .map(|g| (g, self.get(g)))
    }
}
