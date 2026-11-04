use super::theme;
use eframe::egui;

pub fn paint_scene(
    painter: &egui::Painter,
    art: egui::Rect,
    tier: u8,
    variant: u8,
    time: f32,
) {
    paint_sky_ground(painter, art, tier);
    let v = variant % 5;
    match tier {
        1 => match v {
            0 => scene_mine_tunnel(painter, art),
            1 => scene_mountain(painter, art),
            2 => scene_mine_cart(painter, art),
            3 => scene_crystal_cave(painter, art),
            _ => scene_quarry(painter, art),
        },
        2 => match v {
            0 => scene_ship(painter, art, time),
            1 => scene_caravan(painter, art),
            2 => scene_workshop(painter, art),
            3 => scene_harbor(painter, art),
            _ => scene_market_stall(painter, art),
        },
        _ => match v {
            0 => scene_palace(painter, art),
            1 => scene_library(painter, art),
            2 => scene_jewel_hall(painter, art),
            3 => scene_villa(painter, art),
            _ => scene_throne(painter, art),
        },
    }
}

fn paint_sky_ground(painter: &egui::Painter, art: egui::Rect, tier: u8) {
    let mid = art.min.y + art.height() * 0.54;
    let sky = egui::Rect::from_min_max(art.min, egui::pos2(art.max.x, mid));
    let ground = egui::Rect::from_min_max(egui::pos2(art.min.x, mid), art.max);
    let (sky_top, sky_bot, ground_c) = match tier {
        1 => (
            egui::Color32::from_rgb(95, 118, 145),
            egui::Color32::from_rgb(55, 68, 88),
            egui::Color32::from_rgb(52, 44, 38),
        ),
        2 => (
            egui::Color32::from_rgb(110, 135, 168),
            egui::Color32::from_rgb(65, 85, 115),
            egui::Color32::from_rgb(48, 42, 36),
        ),
        _ => (
            egui::Color32::from_rgb(45, 58, 95),
            egui::Color32::from_rgb(28, 38, 68),
            egui::Color32::from_rgb(36, 32, 42),
        ),
    };
    painter.rect_filled(sky, 6.0, sky_bot);
    let upper = egui::Rect::from_min_max(sky.min, egui::pos2(sky.max.x, sky.center().y));
    painter.rect_filled(upper, 6.0, sky_top.gamma_multiply(0.85));
    painter.rect_filled(ground, 6.0, ground_c);
    painter.line_segment(
        [egui::pos2(art.min.x, mid), egui::pos2(art.max.x, mid)],
        egui::Stroke::new(1.0_f32, egui::Color32::BLACK.gamma_multiply(0.3)),
    );
}

fn scene_mine_tunnel(painter: &egui::Painter, art: egui::Rect) {
    let c = art.center_bottom() + egui::vec2(0.0, -8.0);
    let w = art.width() * 0.55;
    let h = art.height() * 0.45;
    let arch = vec![
        c + egui::vec2(-w, 0.0),
        c + egui::vec2(-w * 0.7, -h),
        c + egui::vec2(w * 0.7, -h),
        c + egui::vec2(w, 0.0),
    ];
    painter.add(egui::Shape::convex_polygon(
        arch,
        egui::Color32::from_rgb(25, 22, 20),
        egui::Stroke::NONE,
    ));
    painter.circle_filled(c + egui::vec2(0.0, -h * 0.55), 8.0, theme::accent_gold().gamma_multiply(0.7));
}

fn scene_mountain(painter: &egui::Painter, art: egui::Rect) {
    let base = art.center_bottom().y - 6.0;
    for (cx, h, c) in [
        (0.0, 0.55, egui::Color32::from_rgb(90, 95, 105)),
        (-0.28, 0.42, egui::Color32::from_rgb(70, 75, 85)),
        (0.3, 0.38, egui::Color32::from_rgb(80, 85, 95)),
    ] {
        let peak = egui::pos2(art.center().x + art.width() * cx, base - art.height() * h);
        let tri = vec![
            peak,
            egui::pos2(peak.x - art.width() * 0.22, base),
            egui::pos2(peak.x + art.width() * 0.22, base),
        ];
        painter.add(egui::Shape::convex_polygon(tri, c, egui::Stroke::NONE));
    }
}

fn scene_mine_cart(painter: &egui::Painter, art: egui::Rect) {
    let y = art.center_bottom().y - 14.0;
    let body = egui::Rect::from_center_size(
        egui::pos2(art.center().x, y - 10.0),
        egui::vec2(art.width() * 0.5, 16.0),
    );
    painter.rect_filled(body, 3.0, egui::Color32::from_rgb(65, 50, 38));
    painter.circle_filled(body.left_bottom() + egui::vec2(6.0, 8.0), 5.0, egui::Color32::from_gray(40));
    painter.circle_filled(body.right_bottom() + egui::vec2(-6.0, 8.0), 5.0, egui::Color32::from_gray(40));
    for i in 0..3 {
        painter.circle_filled(
            body.center() + egui::vec2(-12.0 + i as f32 * 12.0, -2.0),
            4.0,
            theme::accent_gold().gamma_multiply(0.85),
        );
    }
}

fn scene_crystal_cave(painter: &egui::Painter, art: egui::Rect) {
    let base = art.max.y - 10.0;
    for i in 0..5 {
        let x = art.min.x + art.width() * (0.15 + i as f32 * 0.17);
        let h = 20.0 + (i % 3) as f32 * 12.0;
        let pts = vec![
            egui::pos2(x - 8.0, base),
            egui::pos2(x, base - h),
            egui::pos2(x + 8.0, base),
        ];
        let col = egui::Color32::from_rgb(120, 180, 200).gamma_multiply(0.55 + (i % 2) as f32 * 0.2);
        painter.add(egui::Shape::convex_polygon(pts, col, egui::Stroke::NONE));
    }
}

fn scene_quarry(painter: &egui::Painter, art: egui::Rect) {
    let steps = 4;
    for i in 0..steps {
        let y = art.min.y + art.height() * (0.55 + i as f32 * 0.1);
        painter.line_segment(
            [
                egui::pos2(art.min.x + 4.0, y),
                egui::pos2(art.max.x - 4.0, y + 6.0),
            ],
            egui::Stroke::new(1.5_f32, egui::Color32::from_gray(70)),
        );
    }
}

fn scene_ship(painter: &egui::Painter, art: egui::Rect, time: f32) {
    let wave = (time * 1.2).sin() * 2.0;
    let hull = egui::Rect::from_center_size(
        art.center_bottom() + egui::vec2(0.0, -18.0 + wave),
        egui::vec2(art.width() * 0.65, 14.0),
    );
    painter.rect_filled(hull, 2.0, egui::Color32::from_rgb(55, 42, 35));
    let mast = hull.center_top() + egui::vec2(0.0, -2.0);
    painter.line_segment(
        [mast, mast + egui::vec2(0.0, -art.height() * 0.35)],
        egui::Stroke::new(2.0_f32, egui::Color32::from_rgb(80, 65, 50)),
    );
    painter.line_segment(
        [mast, mast + egui::vec2(art.width() * 0.28, art.height() * 0.12)],
        egui::Stroke::new(1.5_f32, egui::Color32::WHITE.gamma_multiply(0.5)),
    );
}

fn scene_caravan(painter: &egui::Painter, art: egui::Rect) {
    let y = art.center_bottom().y - 16.0;
    for (dx, h) in [(-22.0, 14.0), (0.0, 18.0), (22.0, 12.0)] {
        let cx = art.center().x + dx;
        painter.add(egui::Shape::convex_polygon(
            vec![
                egui::pos2(cx, y - h),
                egui::pos2(cx - 10.0, y),
                egui::pos2(cx + 10.0, y),
            ],
            egui::Color32::from_rgb(120, 90, 60),
            egui::Stroke::NONE,
        ));
    }
}

fn scene_workshop(painter: &egui::Painter, art: egui::Rect) {
    let table = egui::Rect::from_center_size(
        art.center() + egui::vec2(0.0, art.height() * 0.12),
        egui::vec2(art.width() * 0.7, 10.0),
    );
    painter.rect_filled(table, 2.0, egui::Color32::from_rgb(70, 55, 42));
    painter.rect_filled(
        egui::Rect::from_center_size(table.center() + egui::vec2(0.0, -18.0), egui::vec2(16.0, 12.0)),
        2.0,
        egui::Color32::from_rgb(90, 90, 95),
    );
    painter.circle_filled(table.center() + egui::vec2(8.0, -4.0), 5.0, theme::accent_gold());
}

fn scene_harbor(painter: &egui::Painter, art: egui::Rect) {
    let water_y = art.min.y + art.height() * 0.58;
    painter.rect_filled(
        egui::Rect::from_min_max(egui::pos2(art.min.x, water_y), art.max),
        0.0,
        egui::Color32::from_rgb(40, 70, 95).gamma_multiply(0.85),
    );
    for i in 0..3 {
        let x = art.min.x + 12.0 + i as f32 * 28.0;
        painter.line_segment(
            [egui::pos2(x, water_y - 4.0), egui::pos2(x, art.min.y + 18.0)],
            egui::Stroke::new(1.5_f32, egui::Color32::from_gray(60)),
        );
    }
}

fn scene_market_stall(painter: &egui::Painter, art: egui::Rect) {
    let stall = egui::Rect::from_center_size(
        art.center_bottom() + egui::vec2(0.0, -22.0),
        egui::vec2(art.width() * 0.75, 28.0),
    );
    painter.rect_filled(stall, 2.0, egui::Color32::from_rgb(85, 60, 45));
    let awning = vec![
        stall.left_top(),
        stall.right_top(),
        stall.right_top() + egui::vec2(0.0, -14.0),
        stall.left_top() + egui::vec2(0.0, -10.0),
    ];
    painter.add(egui::Shape::convex_polygon(
        awning,
        theme::brass().gamma_multiply(0.55),
        egui::Stroke::NONE,
    ));
}

fn scene_palace(painter: &egui::Painter, art: egui::Rect) {
    let base = art.center_bottom().y - 8.0;
    let body = egui::Rect::from_min_max(
        egui::pos2(art.center().x - art.width() * 0.35, base - 50.0),
        egui::pos2(art.center().x + art.width() * 0.35, base),
    );
    painter.rect_filled(body, 2.0, egui::Color32::from_rgb(140, 130, 115));
    for dx in [-0.28, 0.0, 0.28] {
        let cx = art.center().x + art.width() * dx;
        painter.rect_filled(
            egui::Rect::from_center_size(
                egui::pos2(cx, base - 58.0),
                egui::vec2(8.0, 22.0),
            ),
            1.0,
            egui::Color32::from_rgb(180, 170, 150),
        );
    }
    painter.add(egui::Shape::convex_polygon(
        vec![
            egui::pos2(art.center().x, body.min.y - 16.0),
            egui::pos2(body.min.x, body.min.y),
            egui::pos2(body.max.x, body.min.y),
        ],
        theme::accent_gold().gamma_multiply(0.45),
        egui::Stroke::NONE,
    ));
}

fn scene_library(painter: &egui::Painter, art: egui::Rect) {
    let shelf = egui::Rect::from_center_size(
        art.center() + egui::vec2(0.0, 8.0),
        egui::vec2(art.width() * 0.8, art.height() * 0.55),
    );
    painter.rect_filled(shelf, 4.0, egui::Color32::from_rgb(55, 45, 38));
    for row in 0..4 {
        let y = shelf.min.y + 8.0 + row as f32 * 14.0;
        for col in 0..5 {
            let x = shelf.min.x + 6.0 + col as f32 * 13.0;
            painter.rect_filled(
                egui::Rect::from_min_size(egui::pos2(x, y), egui::vec2(10.0, 10.0)),
                1.0,
                egui::Color32::from_rgb(100 + col * 8, 80, 70),
            );
        }
    }
}

fn scene_jewel_hall(painter: &egui::Painter, art: egui::Rect) {
    painter.circle_filled(art.center() + egui::vec2(0.0, 4.0), 22.0, egui::Color32::BLACK.gamma_multiply(0.35));
    for i in 0..6 {
        let a = std::f32::consts::TAU * i as f32 / 6.0 - std::f32::consts::FRAC_PI_2;
        let p = art.center() + egui::vec2(a.cos() * 16.0, a.sin() * 16.0 + 4.0);
        painter.circle_filled(p, 4.0, theme::accent_gold());
    }
}

fn scene_villa(painter: &egui::Painter, art: egui::Rect) {
    let house = egui::Rect::from_center_size(
        art.center_bottom() + egui::vec2(0.0, -28.0),
        egui::vec2(art.width() * 0.6, 36.0),
    );
    painter.rect_filled(house, 2.0, egui::Color32::from_rgb(130, 115, 95));
    painter.add(egui::Shape::convex_polygon(
        vec![
            house.left_top() + egui::vec2(-4.0, 0.0),
            house.right_top() + egui::vec2(4.0, 0.0),
            house.center_top() + egui::vec2(0.0, -20.0),
        ],
        egui::Color32::from_rgb(90, 50, 45),
        egui::Stroke::NONE,
    ));
    painter.rect_filled(
        egui::Rect::from_center_size(house.center_bottom() + egui::vec2(0.0, -10.0), egui::vec2(12.0, 18.0)),
        1.0,
        egui::Color32::from_rgb(40, 30, 25),
    );
}

fn scene_throne(painter: &egui::Painter, art: egui::Rect) {
    let seat = egui::Rect::from_center_size(
        art.center_bottom() + egui::vec2(0.0, -20.0),
        egui::vec2(28.0, 32.0),
    );
    painter.rect_filled(seat, 4.0, egui::Color32::from_rgb(100, 20, 30));
    painter.rect_filled(
        egui::Rect::from_center_size(seat.center_top() + egui::vec2(0.0, -8.0), egui::vec2(36.0, 14.0)),
        2.0,
        theme::accent_gold().gamma_multiply(0.65),
    );
}
