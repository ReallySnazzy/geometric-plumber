use num::{BigInt, FromPrimitive};
use raylib::prelude::Color;

use super::upgrades::{Upgrade, PurchasableUpgrade};

pub struct Shape {
    pub x: f64,
    pub y: f64,
    pub vel_x: f64,
    pub vel_y: f64,

    pub rot: f32,
    pub rot_vel: f32,

    pub sides: i32,
    pub color: Color,

    pub laser_cuts: i32,
}

pub struct GamePipe {
    // Number of sides for generated shape
    pub sides: i32,
    pub color: Color,
    pub speed: u64,
    pub last_drop: u64,
}

pub struct GameState {
    pub pipes: Vec<GamePipe>,
    pub shapes: Vec<Shape>,
    pub last_tick: u64,
    pub score: BigInt,
    pub upgrades: Vec<Upgrade>,
    pub available_upgrades: Vec<PurchasableUpgrade>,
    pub mouse_click_x: i32,
    pub mouse_click_y: i32
}

pub fn initial_state()-> GameState {
    GameState { 
        pipes: vec![
            GamePipe { sides: 3, color: Color::GRAY, speed: 1, last_drop: 0 },
        ],
        last_tick: 0,
        shapes: vec!(),
        score: BigInt::from_u64(0u64).unwrap(),
        upgrades: vec!(),
        available_upgrades: vec!(),
        mouse_click_x: 0,
        mouse_click_y: 0,
    }
}