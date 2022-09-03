use num::{BigInt, FromPrimitive};
use raylib::prelude::Color;

use crate::drawing::HEIGHT;

use super::{state::{GameState, Shape}, upgrades::Upgrade};

pub fn shape_tick(state: &mut GameState, dt: u64) {
    let dt = dt as f64 / 1000f64;
    let gravity = 90f64;
    for shape in &mut state.shapes {
        shape.x += shape.vel_x*dt;
        shape.y += shape.vel_y*dt;
        shape.vel_y += dt*gravity;
        shape.vel_x *= (1f64-dt*4f64).max(0f64);
        shape.rot += shape.rot_vel*dt as f32;
    } 
    if state.upgrades.contains(&Upgrade::ShapeLaser(1)) {
        for i in 0..state.shapes.len() {
            if state.shapes[i].laser_cuts == 0 
                && state.shapes[i].y as i32 > HEIGHT/2 + 150 
            {
                state.shapes[i].laser_cuts += 1;
                state.shapes[i].sides += 1;
            }
        }
    }
    for i in (0..state.shapes.len()).rev() {
        if state.shapes[i].y > HEIGHT as f64 + 100f64 {
            state.score += shape_price(&state.shapes[i]);
            state.shapes.remove(i);
        }
    }
}

fn shape_price(shape: &Shape) -> BigInt {
    let side_level = (shape.sides - 2) as u32;
    let side_price = BigInt::from_u64(500).unwrap().pow(side_level) 
        - BigInt::from_u64(470).unwrap();
    let color_multiplier = match shape.color {
        Color::GRAY => BigInt::from_u64(1).unwrap(),
        Color::RED => BigInt::from_u64(10).unwrap(),
        Color::GREEN => BigInt::from_u64(100).unwrap(),
        Color::BLUE => BigInt::from_u64(1000).unwrap(),
        Color::YELLOW => BigInt::from_u64(10_000).unwrap(),
        Color::PURPLE =>  BigInt::from_u64(100_000).unwrap(),
        _ => BigInt::from_u64(0).unwrap()
    };
    return side_price * color_multiplier;
}