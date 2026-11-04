use crate::game::{GameError, LogEntry};
use crate::gem::Gem;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Language {
    #[default]
    En,
    Ko,
    Ja,
    Zh,
}

impl Language {
    pub const ALL: [Language; 4] = [Language::Ko, Language::En, Language::Ja, Language::Zh];

    pub fn label(self) -> &'static str {
        match self {
            Language::Ko => "한국어",
            Language::En => "English",
            Language::Ja => "日本語",
            Language::Zh => "中文",
        }
    }
}

#[derive(Clone, Copy)]
pub struct Locale {
    pub lang: Language,
}

impl Locale {
    pub fn new(lang: Language) -> Self {
        Self { lang }
    }

    pub fn app_title(&self) -> &'static str {
        match self.lang {
            Language::Ko => "젬스미스",
            Language::En => "Gemsmith",
            Language::Ja => "ジェムスミス",
            Language::Zh => "宝石匠",
        }
    }

    pub fn win_goal_subtitle(&self, pts: u8) -> String {
        match self.lang {
            Language::Ko => format!("{pts}점 먼저 → 마지막 한 바퀴"),
            Language::En => format!("First to {pts} pts — then one final round"),
            Language::Ja => format!("{pts}点先取 → 最終ラウンド"),
            Language::Zh => format!("先达到{pts}分 → 最后一轮"),
        }
    }

    pub fn language_label(&self) -> &'static str {
        match self.lang {
            Language::Ko => "언어",
            Language::En => "Language",
            Language::Ja => "言語",
            Language::Zh => "语言",
        }
    }

    pub fn new_game(&self) -> &'static str {
        match self.lang {
            Language::Ko => "새 게임",
            Language::En => "New game",
            Language::Ja => "新しいゲーム",
            Language::Zh => "新游戏",
        }
    }

    pub fn cpu_difficulty(&self) -> &'static str {
        match self.lang {
            Language::Ko => "CPU 난이도",
            Language::En => "CPU difficulty",
            Language::Ja => "CPU難易度",
            Language::Zh => "CPU难度",
        }
    }

    pub fn difficulty_easy(&self) -> &'static str {
        match self.lang {
            Language::Ko => "쉬움",
            Language::En => "Easy",
            Language::Ja => "易しい",
            Language::Zh => "简单",
        }
    }

    pub fn difficulty_medium(&self) -> &'static str {
        match self.lang {
            Language::Ko => "보통",
            Language::En => "Medium",
            Language::Ja => "普通",
            Language::Zh => "普通",
        }
    }

    pub fn difficulty_hard(&self) -> &'static str {
        match self.lang {
            Language::Ko => "어려움",
            Language::En => "Hard",
            Language::Ja => "難しい",
            Language::Zh => "困难",
        }
    }

    pub fn play(&self) -> &'static str {
        match self.lang {
            Language::Ko => "  플레이  ",
            Language::En => "  Play  ",
            Language::Ja => "  プレイ  ",
            Language::Zh => "  开始  ",
        }
    }

    pub fn back_to_menu(&self) -> &'static str {
        match self.lang {
            Language::Ko => "메뉴로",
            Language::En => "Back to menu",
            Language::Ja => "メニューへ",
            Language::Zh => "返回菜单",
        }
    }

    pub fn quit_to_menu(&self) -> &'static str {
        match self.lang {
            Language::Ko => "메뉴 나가기",
            Language::En => "Quit to menu",
            Language::Ja => "メニューに戻る",
            Language::Zh => "退出到菜单",
        }
    }

    pub fn your_turn(&self) -> &'static str {
        match self.lang {
            Language::Ko => "당신 차례입니다.",
            Language::En => "Your turn.",
            Language::Ja => "あなたの番です。",
            Language::Zh => "轮到你了。",
        }
    }

    pub fn winner_msg(&self, name: &str, pts: u8) -> String {
        match self.lang {
            Language::Ko => format!("{name} 승리 — {pts}점"),
            Language::En => format!("{name} wins — {pts} points"),
            Language::Ja => format!("{name} の勝ち — {pts}点"),
            Language::Zh => format!("{name} 获胜 — {pts}分"),
        }
    }

    pub fn player_you(&self) -> &'static str {
        match self.lang {
            Language::Ko => "나",
            Language::En => "You",
            Language::Ja => "あなた",
            Language::Zh => "你",
        }
    }

    pub fn player_cpu(&self) -> &'static str {
        match self.lang {
            Language::Ko => "CPU",
            Language::En => "CPU",
            Language::Ja => "CPU",
            Language::Zh => "CPU",
        }
    }

    pub fn gem_bank(&self) -> &'static str {
        match self.lang {
            Language::Ko => "보석 은행",
            Language::En => "Gem bank",
            Language::Ja => "宝石バンク",
            Language::Zh => "宝石银行",
        }
    }

    pub fn development_cards(&self) -> &'static str {
        match self.lang {
            Language::Ko => "발전 카드",
            Language::En => "Development cards",
            Language::Ja => "発展カード",
            Language::Zh => "发展卡",
        }
    }

    pub fn tier_short(&self, tier: u8) -> String {
        match self.lang {
            Language::Ko => format!("{tier}단"),
            Language::En => format!("T{tier}"),
            Language::Ja => format!("T{tier}"),
            Language::Zh => format!("{tier}级"),
        }
    }

    pub fn tier_label(&self, tier: u8) -> String {
        match self.lang {
            Language::Ko => format!("{tier}티어"),
            Language::En => format!("Tier {tier}"),
            Language::Ja => format!("ティア{tier}"),
            Language::Zh => format!("{tier}级"),
        }
    }

    pub fn patrons(&self) -> &'static str {
        match self.lang {
            Language::Ko => "후원자",
            Language::En => "Patrons",
            Language::Ja => "パトロン",
            Language::Zh => "赞助人",
        }
    }

    pub fn patron_title(&self) -> &'static str {
        match self.lang {
            Language::Ko => "후원자",
            Language::En => "Patron",
            Language::Ja => "パトロン",
            Language::Zh => "赞助人",
        }
    }

    pub fn patron_points(&self) -> &'static str {
        match self.lang {
            Language::Ko => "+3 승점",
            Language::En => "+3 victory points",
            Language::Ja => "+3勝利点",
            Language::Zh => "+3胜利点",
        }
    }

    pub fn requires(&self) -> &'static str {
        match self.lang {
            Language::Ko => "조건:",
            Language::En => "Requires:",
            Language::Ja => "条件:",
            Language::Zh => "需要:",
        }
    }

    pub fn cost(&self) -> &'static str {
        match self.lang {
            Language::Ko => "비용",
            Language::En => "Cost",
            Language::Ja => "コスト",
            Language::Zh => "费用",
        }
    }

    pub fn points_fmt(&self, pts: u8) -> String {
        match self.lang {
            Language::Ko => format!("{pts}점"),
            Language::En => format!("{pts} pt"),
            Language::Ja => format!("{pts}点"),
            Language::Zh => format!("{pts}分"),
        }
    }

    pub fn points_gain(&self, pts: u8) -> String {
        match self.lang {
            Language::Ko => format!("+{pts}"),
            Language::En => format!("+{pts}"),
            Language::Ja => format!("+{pts}"),
            Language::Zh => format!("+{pts}"),
        }
    }

    pub fn tokens(&self) -> &'static str {
        match self.lang {
            Language::Ko => "토큰",
            Language::En => "Tokens",
            Language::Ja => "トークン",
            Language::Zh => "筹码",
        }
    }

    pub fn bonuses(&self) -> &'static str {
        match self.lang {
            Language::Ko => "보너스",
            Language::En => "Bonuses",
            Language::Ja => "ボーナス",
            Language::Zh => "奖励",
        }
    }

    pub fn reserved(&self) -> &'static str {
        match self.lang {
            Language::Ko => "예약",
            Language::En => "Reserved",
            Language::Ja => "予約",
            Language::Zh => "保留",
        }
    }

    pub fn buy(&self) -> &'static str {
        match self.lang {
            Language::Ko => "구매",
            Language::En => "Buy",
            Language::Ja => "購入",
            Language::Zh => "购买",
        }
    }

    pub fn hold(&self) -> &'static str {
        match self.lang {
            Language::Ko => "예약",
            Language::En => "Hold",
            Language::Ja => "確保",
            Language::Zh => "保留",
        }
    }

    pub fn take_gems_help(&self) -> &'static str {
        match self.lang {
            Language::Ko => "보석 가져오기 — 같은 색 2개 또는 다른 색 3개 선택 후 가져오기",
            Language::En => "Take gems — pick 2 same or 3 different, then Take",
            Language::Ja => "宝石を取る — 同色2個または異色3個を選んで取得",
            Language::Zh => "取宝石 — 选2个同色或3个不同色，然后取",
        }
    }

    pub fn take_selected(&self) -> &'static str {
        match self.lang {
            Language::Ko => "가져오기",
            Language::En => "Take selected",
            Language::Ja => "取得",
            Language::Zh => "取走",
        }
    }

    pub fn clear(&self) -> &'static str {
        match self.lang {
            Language::Ko => "지우기",
            Language::En => "Clear",
            Language::Ja => "クリア",
            Language::Zh => "清除",
        }
    }

    pub fn reserve_from_deck(&self) -> &'static str {
        match self.lang {
            Language::Ko => "덱에서 예약:",
            Language::En => "Reserve from deck:",
            Language::Ja => "山札から予約:",
            Language::Zh => "从牌堆保留:",
        }
    }

    pub fn reserve_deck_btn(&self, tier: u8) -> String {
        self.tier_label(tier)
    }

    pub fn gem_name(&self, g: Gem) -> &'static str {
        match g {
            Gem::Ruby => match self.lang {
                Language::Ko => "루비",
                Language::En => "ruby",
                Language::Ja => "ルビー",
                Language::Zh => "红宝石",
            },
            Gem::Sapphire => match self.lang {
                Language::Ko => "사파이어",
                Language::En => "sapphire",
                Language::Ja => "サファイア",
                Language::Zh => "蓝宝石",
            },
            Gem::Emerald => match self.lang {
                Language::Ko => "에메랄드",
                Language::En => "emerald",
                Language::Ja => "エメラルド",
                Language::Zh => "翡翠",
            },
            Gem::Diamond => match self.lang {
                Language::Ko => "다이아",
                Language::En => "diamond",
                Language::Ja => "ダイヤ",
                Language::Zh => "钻石",
            },
            Gem::Onyx => match self.lang {
                Language::Ko => "오닉스",
                Language::En => "onyx",
                Language::Ja => "オニキス",
                Language::Zh => "玛瑙",
            },
            Gem::Gold => match self.lang {
                Language::Ko => "골드",
                Language::En => "gold",
                Language::Ja => "ゴールド",
                Language::Zh => "黄金",
            },
        }
    }

    pub fn format_error(&self, err: &GameError) -> String {
        match err {
            GameError::MaxTokens { would } => match self.lang {
                Language::Ko => format!("토큰은 최대 10개 (선택 시 {would}개)"),
                Language::En => format!("max 10 tokens (would be {would})"),
                Language::Ja => format!("トークンは最大10個（{would}個になる）"),
                Language::Zh => format!("筹码最多10个（将达到{would}个）"),
            },
            GameError::BankEmpty => self.msg(
                "은행에 보석이 없습니다",
                "bank empty",
                "バンクが空です",
                "银行已空",
            ),
            GameError::EmptySlot => self.msg("빈 칸입니다", "empty slot", "空のスロット", "空位"),
            GameError::BadReserveIndex => self.msg(
                "잘못된 예약 번호",
                "bad reserve index",
                "予約番号が不正",
                "保留索引无效",
            ),
            GameError::CannotAfford => self.msg(
                "비용을 낼 수 없습니다",
                "cannot afford",
                "支払えません",
                "无法支付",
            ),
            GameError::ReserveFull => self.msg(
                "예약 슬롯이 가득 찼습니다",
                "reserve full",
                "予約がいっぱい",
                "保留位已满",
            ),
            GameError::DeckEmpty => self.msg("덱이 비었습니다", "deck empty", "山札が空", "牌堆已空"),
            GameError::PickAtLeastOne => self.msg(
                "보석을 하나 이상 고르세요",
                "pick at least one gem",
                "宝石を1つ以上選んでください",
                "请至少选一颗宝石",
            ),
            GameError::TakeTwoOrThree => self.msg(
                "같은 색 2개 또는 다른 색 3개",
                "take 2 same or 3 different",
                "同色2個または異色3個",
                "取2个同色或3个不同色",
            ),
            GameError::TwoSameColor => self.msg(
                "두 개는 같은 색이어야 합니다",
                "two must be same color",
                "2個は同色である必要があります",
                "两个必须同色",
            ),
            GameError::NeedFourInBank => self.msg(
                "같은 색 2개는 은행에 4개 이상 필요",
                "need 4+ in bank for two",
                "同色2個はバンクに4個以上必要",
                "取2个同色需要银行至少4个",
            ),
            GameError::NoGoldOnTake => self.msg(
                "가져올 때 골드 불가",
                "no gold on take",
                "取得時ゴールド不可",
                "取宝石时不能选黄金",
            ),
            GameError::ThreeMustDiffer => self.msg(
                "세 개는 모두 다른 색",
                "three must differ",
                "3個はすべて異なる色",
                "三个必须不同色",
            ),
            GameError::MaxThreeGems => self.msg("최대 3개", "max 3 gems", "最大3個", "最多3颗"),
        }
    }

    fn msg(&self, ko: &str, en: &str, ja: &str, zh: &str) -> String {
        match self.lang {
            Language::Ko => ko,
            Language::En => en,
            Language::Ja => ja,
            Language::Zh => zh,
        }
        .to_string()
    }

    pub fn format_log(&self, entry: &LogEntry) -> String {
        match entry {
            LogEntry::Took { name, gems } => match self.lang {
                Language::Ko => format!("{name} — 보석 {gems} 획득"),
                Language::En => format!("{name} took {gems}"),
                Language::Ja => format!("{name} が {gems} を取得"),
                Language::Zh => format!("{name} 取得了 {gems}"),
            },
            LogEntry::ReservedTable { name, tier } => match self.lang {
                Language::Ko => format!("{name} — {tier}티어 카드 예약"),
                Language::En => format!("{name} reserved a tier-{tier} card"),
                Language::Ja => format!("{name} がティア{tier}を予約"),
                Language::Zh => format!("{name} 保留了{tier}级卡"),
            },
            LogEntry::ReservedDeck { name, tier } => match self.lang {
                Language::Ko => format!("{name} — {tier}티어 덱에서 예약"),
                Language::En => format!("{name} reserved from tier-{tier} deck"),
                Language::Ja => format!("{name} がティア{tier}山札から予約"),
                Language::Zh => format!("{name} 从{tier}级牌堆保留"),
            },
            LogEntry::Bought {
                name,
                tier,
                points,
                bonus,
            } => {
                let gem = self.gem_name(*bonus);
                match self.lang {
                    Language::Ko => {
                        format!("{name} — {tier}티어 구매 +{points}점 ({gem})")
                    }
                    Language::En => format!(
                        "{name} bought tier-{tier} +{points}pt bonus {bonus}"
                    ),
                    Language::Ja => {
                        format!("{name} — T{tier}購入 +{points}点 ({gem})")
                    }
                    Language::Zh => format!("{name} 购买{tier}级 +{points}分 ({gem})"),
                }
            }
            LogEntry::FinalRound { name } => match self.lang {
                Language::Ko => format!("{name} — {WIN_PTS}점 돌파, 마지막 라운드"),
                Language::En => format!("{name} hit {WIN_PTS} points — final round"),
                Language::Ja => format!("{name} — {WIN_PTS}点到達、最終ラウンド"),
                Language::Zh => format!("{name} 达到{WIN_PTS}分 — 最后一轮"),
            },
            LogEntry::Patron { name, points } => match self.lang {
                Language::Ko => format!("{name} — 후원자 +{points}점"),
                Language::En => format!("{name} gained patron +{points}pt"),
                Language::Ja => format!("{name} — パトロン +{points}点"),
                Language::Zh => format!("{name} 获得赞助人 +{points}分"),
            },
        }
    }
}

const WIN_PTS: u8 = crate::game::WIN_POINTS;
