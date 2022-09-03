use num::BigInt;
use raylib::{RaylibHandle, RaylibThread, prelude::{Color, RaylibDrawHandle, RaylibDraw, Vector2}};

use crate::{game_logic::{state::GameState, upgrades::Upgrade}, assets::LoadedAssets};

pub const WIDTH: i32 = 1280;
pub const HEIGHT: i32 = 720;

pub fn draw(rl: &mut RaylibHandle, thread: &RaylibThread, game_state: &GameState, loaded_assets: &LoadedAssets) {
    let mut d = rl.begin_drawing(&thread);         
    d.clear_background(Color::new(220, 220, 240, 255));
    draw_shapes(&mut d, game_state);
    draw_pipes(loaded_assets, &mut d, game_state);
    draw_shape_upgrade_lasers(&mut d, game_state);
    draw_ui(loaded_assets, &mut d, game_state);
}

fn draw_shape_upgrade_lasers(d: &mut RaylibDrawHandle, game_state: &GameState) {
    if game_state.upgrades.contains(&Upgrade::ShapeLaser(1)) {
        d.draw_rectangle(0, HEIGHT/2 + 150, WIDTH, 7, Color::RED);
        d.draw_rectangle(0, HEIGHT/2+2 + 150, WIDTH, 3, Color::new(255, 100, 100, 255));
    }
}

fn draw_shapes(d: &mut RaylibDrawHandle, game_state: &GameState) {
    for shape in &game_state.shapes {
        d.draw_poly(
            Vector2::new(shape.x as f32, shape.y as f32), 
            shape.sides, 
            25f32, 
            shape.rot, 
            shape.color
        );
    }
}

fn draw_pipes(loaded_assets: &LoadedAssets, d: &mut RaylibDrawHandle, game_state: &GameState) {
    for (i, _pipe) in game_state.pipes.iter().enumerate().rev() {
        let i = i as i32;
        let darkness: u8 = (255 - i*20).try_into().unwrap();
        d.draw_texture(
            &loaded_assets.pipe_texture,
            30 + 120*i,
            30 + 10*i,
            &Color::new(darkness, darkness, darkness, 255)
        );
    }
}

fn draw_ui(loaded_assets: &LoadedAssets, d: &mut RaylibDrawHandle, game_state: &GameState) {
    let mut y = 32;
    d.draw_rectangle(WIDTH/2, 0, WIDTH/2, HEIGHT, Color::new(70, 70, 100, 255));
    d.draw_text(&format_score(game_state), WIDTH/2 + 32, y, 24, Color::WHITE);
    y += 24 + 12;
    d.draw_text("Upgrades", WIDTH/2 + 32, y, 32, Color::WHITE);
    y += 32 + 12;
    for upgrade in &game_state.available_upgrades {
        d.draw_text(&upgrade.description, WIDTH/2 + 32, y + 10, 16, Color::WHITE);
        let box_color = if upgrade.price > game_state.score {
            Color::new(40, 120, 40, 255)
        } else {
            Color::new(80, 180, 80, 255)
        };
        let box_text_color = if upgrade.price > game_state.score {
            Color::GRAY
        } else {
            Color::WHITE
        };
        d.draw_rectangle(WIDTH - 50 - 32, y, 50, 36, box_color);
        d.draw_text("Buy", WIDTH - 50 - 32 + 10, y + 10, 16, box_text_color);
        d.draw_text(&format_num(&upgrade.price, true), WIDTH - 50 - 32 - 80 - 12, y + 10, 16, Color::WHITE);
        y += 36 + 12;
    }
}

const NUM_ABBREVIATIONS: [(usize, &'static str, &'static str); 22] = [
    (66, "Unvigintillion", "c"),
    (63, "Vigintillion", "v"),
    (60, "Novemdecillion", "N"),
    (57, "Octodecillion", "O"),
    (54, "Septendecillion", "St"),
    (51, "Sexdecillion", "Sd"),
    (48, "Quindecillion", "Qd"),
    (45, "Quattuordecillion", "Qt"),
    (42, "Tredecillion", "T"),
    (39, "Duodecillion", "D"),
    (36, "Undecillion", "U"),
    (33, "Decillion", "d"),
    (30, "Nonillion", "n"),
    (27, "Octillion", "o"),
    (24, "Septillion", "S"),
    (21, "Sextillion", "s"),
    (18, "Quintillion", "Q"),
    (15, "Quadrillion", "q"),
    (12, "Trillion", "t"),
    (9, "Billion", "B"),
    (6, "Million", "M"),
    (3, "Thousand", "K"),
];

fn format_score(state: &GameState) -> String {
    format!("Score: {}", format_num(&state.score, false))
}

fn format_num(num: &BigInt, short: bool) -> String {
    let raw_num_str = num.to_str_radix(10);
    for abbr in NUM_ABBREVIATIONS {
        if raw_num_str.len() > abbr.0 + 1 {
            let abbr_text = match short {
                true => abbr.2,
                false => abbr.1
            };
            unsafe {
                return format!(
                    "{}.{} {}", 
                    raw_num_str.get_unchecked(..raw_num_str.len() - abbr.0), 
                    raw_num_str.get_unchecked(raw_num_str.len() - abbr.0..raw_num_str.len() - abbr.0 + 1), 
                    abbr_text)
            }
        }
    }
    return raw_num_str;
}