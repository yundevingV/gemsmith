use super::draw::{self, GemChipOpts};
use super::fonts;
use super::layout;
use super::theme;
use eframe::egui;
use gemsmith::ai::{pick_action, Difficulty};
use gemsmith::card::can_afford;
use gemsmith::game::{Action, Game, TABLE_SIZE, WIN_POINTS};
use gemsmith::gem::{Gem, GemCounts};
use gemsmith::locale::{Language, Locale};
use rand::SeedableRng;
use rand::rngs::StdRng;

enum Screen {
    Menu,
    Play,
}

pub struct GemsmithApp {
    screen: Screen,
    language: Language,
    difficulty: Difficulty,
    game: Option<Game>,
    status: String,
    gem_pick: Vec<Gem>,
    focus_window_once: bool,
}

impl GemsmithApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        fonts::setup(&cc.egui_ctx);
        theme::apply(&cc.egui_ctx);
        Self {
            screen: Screen::Menu,
            language: Language::Ko,
            difficulty: Difficulty::Medium,
            game: None,
            status: String::new(),
            gem_pick: Vec::new(),
            focus_window_once: true,
        }
    }

    fn loc(&self) -> Locale {
        Locale::new(self.language)
    }

    fn new_game(&mut self) {
        let loc = self.loc();
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(42);
        self.game = Some(Game::new_human_vs_ai(
            seed,
            loc.player_you(),
            loc.player_cpu(),
        ));
        self.gem_pick.clear();
        self.status = loc.your_turn().into();
        self.screen = Screen::Play;
    }

    fn run_cpu_turns(&mut self) {
        let loc = self.loc();
        let Some(game) = self.game.as_mut() else {
            return;
        };
        let diff = self.difficulty;
        while !game.is_over() && !game.current_player().is_human {
            let mut rng = StdRng::seed_from_u64(game.seed + game.log.len() as u64);
            let action = pick_action(game, diff, &mut rng);
            if game.apply(action).is_err() {
                if let Some(a) = game.legal_actions().into_iter().next() {
                    let _ = game.apply(a);
                }
            }
        }
        if let Some(game) = self.game.as_ref() {
            if game.is_over() {
                if let Some(w) = game.winner() {
                    self.status = loc.winner_msg(&game.players[w].name, game.players[w].points);
                }
            } else if game.current_player().is_human {
                self.status = loc.your_turn().into();
            } else if let Some(entry) = game.log.last() {
                self.status = loc.format_log(entry);
            }
        }
    }

    fn apply_human(&mut self, action: Action) {
        let loc = self.loc();
        let Some(game) = self.game.as_mut() else {
            return;
        };
        match game.apply(action) {
            Ok(()) => {
                self.gem_pick.clear();
                self.run_cpu_turns();
            }
            Err(e) => self.status = loc.format_error(&e),
        }
    }
}

impl eframe::App for GemsmithApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let loc = self.loc();
        ctx.send_viewport_cmd(egui::ViewportCommand::Title(loc.app_title().into()));

        if matches!(self.screen, Screen::Menu | Screen::Play) {
            ctx.request_repaint_after(std::time::Duration::from_millis(16));
        }

        if self.focus_window_once {
            self.focus_window_once = false;
            ctx.send_viewport_cmd(egui::ViewportCommand::Focus);
            if let Some(cmd) = egui::ViewportCommand::center_on_screen(ctx) {
                ctx.send_viewport_cmd(cmd);
            }
        }

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme::bg_main())
                    .inner_margin(14.0),
            )
            .show(ctx, |ui| {
                let rect = ui.max_rect();
                theme::velvet_overlay(ui.painter(), rect);
                self.top_bar(ui, &loc);
                match self.screen {
                    Screen::Menu => self.menu(ui, &loc),
                    Screen::Play => self.play(ui, &loc),
                }
            });
    }
}

impl GemsmithApp {
    fn top_bar(&self, ui: &mut egui::Ui, loc: &Locale) {
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(loc.app_title())
                    .font(theme::font_display(26.0))
                    .color(theme::accent_gold()),
            );
            ui.label(
                egui::RichText::new(loc.win_goal_subtitle(WIN_POINTS))
                    .small()
                    .color(egui::Color32::GRAY),
            );
        });
        ui.add_space(4.0);
    }

    fn menu(&mut self, ui: &mut egui::Ui, loc: &Locale) {
        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.add_space(8.0);
                draw::menu_hero(ui);
                ui.add_space(16.0);
                egui::Frame::new()
                    .fill(theme::bg_panel())
                    .stroke(egui::Stroke::new(
                        1.5_f32,
                        theme::accent_gold().gamma_multiply(0.5),
                    ))
                    .corner_radius(12.0)
                    .inner_margin(24.0)
                    .show(ui, |ui| {
                        ui.set_min_width(340.0);
                        ui.label(loc.language_label());
                        ui.horizontal_wrapped(|ui| {
                            for lang in Language::ALL {
                                ui.selectable_value(&mut self.language, lang, lang.label());
                            }
                        });
                        ui.add_space(12.0);
                        ui.label(egui::RichText::new(loc.new_game()).size(18.0).strong());
                        ui.add_space(8.0);
                        ui.label(loc.cpu_difficulty());
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut self.difficulty,
                                Difficulty::Easy,
                                loc.difficulty_easy(),
                            );
                            ui.selectable_value(
                                &mut self.difficulty,
                                Difficulty::Medium,
                                loc.difficulty_medium(),
                            );
                            ui.selectable_value(
                                &mut self.difficulty,
                                Difficulty::Hard,
                                loc.difficulty_hard(),
                            );
                        });
                        ui.add_space(16.0);
                        if ui
                            .add(
                                egui::Button::new(
                                    egui::RichText::new(loc.play()).size(16.0).strong(),
                                )
                                .fill(theme::bg_felt())
                                .stroke(egui::Stroke::new(1.5_f32, theme::accent_gold())),
                            )
                            .clicked()
                        {
                            self.new_game();
                        }
                    });
                ui.add_space(24.0);
            });
    }

    fn play(&mut self, ui: &mut egui::Ui, loc: &Locale) {
        let Some(game) = self.game.as_ref() else {
            self.screen = Screen::Menu;
            return;
        };
        if game.is_over() {
            ui.vertical_centered(|ui| {
                ui.add_space(40.0);
                draw::menu_hero(ui);
                ui.add_space(16.0);
                ui.colored_label(theme::accent_gold(), &self.status);
                if ui.button(loc.back_to_menu()).clicked() {
                    self.screen = Screen::Menu;
                    self.game = None;
                }
            });
            return;
        }

        ui.horizontal(|ui| {
            let turn = egui::RichText::new(&self.status).strong();
            if game.current_player().is_human {
                ui.label(turn.color(egui::Color32::from_rgb(120, 200, 140)));
            } else {
                ui.label(turn.color(egui::Color32::GRAY));
            }
        });
        ui.add_space(8.0);

        let human_turn = game.current == 0 && game.current_player().is_human;
        let mut pending: Option<Action> = None;
        let layout = layout::PlayLayout::measure(ui);

        egui::ScrollArea::vertical()
            .id_salt("play_vscroll")
            .auto_shrink([false, false])
            .show(ui, |ui| {
                ui.set_max_width(layout.content_w);
                Self::play_board(ui, loc, game, layout, human_turn, &mut pending);
            });

        if human_turn {
            ui.add_space(8.0);
            theme::sidebar_panel(ui, |ui| {
                ui.strong(loc.take_gems_help());
                ui.horizontal_wrapped(|ui| {
                    for g in Gem::BASIC {
                        let on = self.gem_pick.contains(&g);
                        let r = draw::draw_gem_chip(
                            ui,
                            g,
                            48.0,
                            None,
                            egui::Sense::click(),
                            GemChipOpts {
                                selected: on,
                                ..Default::default()
                            },
                        );
                        if r.clicked() {
                            if on {
                                self.gem_pick.retain(|x| *x != g);
                            } else if self.gem_pick.len() < 3 {
                                self.gem_pick.push(g);
                            }
                        }
                    }
                    if ui.button(loc.take_selected()).clicked() {
                        pending = Some(Action::TakeGems(self.gem_pick.clone()));
                    }
                    if ui.button(loc.clear()).clicked() {
                        self.gem_pick.clear();
                    }
                });
                ui.horizontal_wrapped(|ui| {
                    ui.label(loc.reserve_from_deck());
                    for tier in 1..=3u8 {
                        if ui.button(loc.reserve_deck_btn(tier)).clicked() {
                            pending = Some(Action::ReserveDeck { tier });
                        }
                    }
                });
            });
        }

        ui.add_space(8.0);
        if ui.button(loc.quit_to_menu()).clicked() {
            self.screen = Screen::Menu;
            self.game = None;
        }

        if let Some(a) = pending {
            self.apply_human(a);
        }
    }

    fn play_board(
        ui: &mut egui::Ui,
        loc: &Locale,
        game: &Game,
        layout: layout::PlayLayout,
        human_turn: bool,
        pending: &mut Option<Action>,
    ) {
        if layout.narrow {
            Self::play_main_panel(ui, loc, game, layout.main_w, human_turn, pending);
            ui.add_space(10.0);
            Self::play_side_panel(ui, loc, game, human_turn, pending);
        } else {
            ui.horizontal_top(|ui| {
                ui.vertical(|ui| {
                    ui.set_max_width(layout.main_w);
                    Self::play_main_panel(ui, loc, game, layout.main_w, human_turn, pending);
                });
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    ui.set_min_width(layout.side_w);
                    ui.set_max_width(layout.side_w);
                    Self::play_side_panel(ui, loc, game, human_turn, pending);
                });
            });
        }
    }

    fn play_main_panel(
        ui: &mut egui::Ui,
        loc: &Locale,
        game: &Game,
        main_w: f32,
        human_turn: bool,
        pending: &mut Option<Action>,
    ) {
        theme::board_panel(ui, loc.gem_bank(), |ui| {
            draw::bank_row(ui, &game.bank);
        });
        ui.add_space(10.0);
        theme::board_panel(ui, loc.development_cards(), |ui| {
            for tier in 1..=3u8 {
                Self::tier_market_row(ui, loc, game, tier, human_turn, pending, main_w);
                ui.add_space(6.0);
            }
        });
    }

    fn play_side_panel(
        ui: &mut egui::Ui,
        loc: &Locale,
        game: &Game,
        human_turn: bool,
        pending: &mut Option<Action>,
    ) {
        theme::sidebar_panel(ui, |ui| {
            ui.label(
                egui::RichText::new(loc.patrons())
                    .strong()
                    .color(theme::accent_gold()),
            );
            ui.add_space(6.0);
            for (i, p) in game.patrons.iter().enumerate() {
                draw::patron_tile(ui, i, p, loc);
                ui.add_space(6.0);
            }
        });
        ui.add_space(10.0);
        for (pi, pl) in game.players.iter().enumerate() {
            let active = pi == game.current;
            theme::player_board_frame(ui, active, |ui| {
                let mark = if active { "▶ " } else { "  " };
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(format!("{mark}{} ♛", pl.name))
                            .font(theme::font_body(16.0))
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            egui::RichText::new(loc.points_fmt(pl.points))
                                .font(theme::font_display(20.0))
                                .color(theme::accent_gold()),
                        );
                    });
                });
                ui.label(loc.tokens());
                draw::player_tokens(ui, &pl.tokens);
                ui.label(loc.bonuses());
                draw::player_tokens(ui, &pl.bonuses);
                if !pl.reserved.is_empty() {
                    ui.label(loc.reserved());
                    for (ri, c) in pl.reserved.iter().enumerate() {
                        ui.horizontal(|ui| {
                            draw::mini_card(ui, c, loc);
                            if human_turn && pi == 0 && ui.small_button(loc.buy()).clicked() {
                                *pending = Some(Action::BuyReserved { idx: ri });
                            }
                        });
                    }
                }
            });
            ui.add_space(6.0);
        }
    }

    fn tier_market_row(
        ui: &mut egui::Ui,
        loc: &Locale,
        game: &Game,
        tier: u8,
        human_turn: bool,
        pending: &mut Option<Action>,
        main_w: f32,
    ) {
        ui.horizontal(|ui| {
            ui.set_min_width(34.0);
            ui.label(
                egui::RichText::new(loc.tier_short(tier))
                    .small()
                    .color(theme::tier_accent(tier)),
            );
            let card_w = (main_w - 48.0).max(200.0);
            if layout::tier_row_needs_scroll(card_w) {
                egui::ScrollArea::horizontal()
                    .id_salt(("tier_scroll", tier))
                    .max_width(card_w)
                    .show(ui, |ui| {
                        Self::tier_cards(ui, loc, game, tier, human_turn, pending);
                    });
            } else {
                ui.scope(|ui| {
                    ui.set_max_width(card_w);
                    Self::tier_cards(ui, loc, game, tier, human_turn, pending);
                });
            }
        });
    }

    fn tier_cards(
        ui: &mut egui::Ui,
        loc: &Locale,
        game: &Game,
        tier: u8,
        human_turn: bool,
        pending: &mut Option<Action>,
    ) {
        let pl = &game.players[0];
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            for slot in 0..TABLE_SIZE {
                if let Some(c) = &game.table[(tier - 1) as usize][slot] {
                    let can_buy =
                        human_turn && can_afford(&pl.tokens, &pl.bonuses, &c.cost);
                    let wallet = if human_turn {
                        Some((&pl.tokens as &GemCounts, &pl.bonuses as &GemCounts))
                    } else {
                        None
                    };
                    draw::market_card(
                        ui,
                        c,
                        tier,
                        slot,
                        human_turn,
                        can_buy,
                        wallet,
                        pending,
                        loc,
                    );
                }
            }
        });
    }
}
