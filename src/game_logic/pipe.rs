use std::f32::consts::{E, PI};

use rand::{Rng};
use raylib::{prelude::Color, ffi::log2l};

use super::{state::{GameState, GamePipe, Shape}};
use super::upgrades::Upgrade;

pub fn pipe_tick(game_state: &mut GameState, dt: u64) {
    pipe_speed_tick(game_state);
    pipe_shape_drop_tick(game_state, dt);
    pipe_count_tick(game_state);
}

fn pipe_count_tick(game_state: &mut GameState) {
    if game_state.pipes.len() < 2 && game_state.upgrades.contains(&Upgrade::Pipe2) {
        game_state.pipes.push(GamePipe { 
            sides: 4, 
            color: Color::GRAY, 
            speed: 1, 
            last_drop: 0,
        });
    }
    if game_state.pipes.len() < 3 && game_state.upgrades.contains(&Upgrade::Pipe3) {
        game_state.pipes.push(GamePipe { 
            sides: 5, 
            color: Color::GRAY, 
            speed: 1, 
            last_drop: 0,
        });
    }
    if game_state.pipes.len() < 4 && game_state.upgrades.contains(&Upgrade::Pipe4) {
        game_state.pipes.push(GamePipe { 
            sides: 6, 
            color: Color::GRAY, 
            speed: 1, 
            last_drop: 0,
        });
    }
}

fn pipe_speed_tick(game_state: &mut GameState) {
    game_state.pipes[0].speed = game_state.upgrades.iter().fold(1u64, |p, upg| {
        if let Upgrade::Pipe1Speed(speed) = upg {
            if speed > &p {
                return speed.clone();
            }
        }
        return p;
    });
    if game_state.pipes.len() >= 2 {
        game_state.pipes[1].speed = game_state.upgrades.iter().fold(1u64, |p, upg| {
            if let Upgrade::Pipe2Speed(speed) = upg {
                if speed > &p {
                    return speed.clone();
                }
            }
            return p;
        });
    }
    if game_state.pipes.len() >= 3 {
        game_state.pipes[2].speed = game_state.upgrades.iter().fold(1u64, |p, upg| {
            if let Upgrade::Pipe3Speed(speed) = upg {
                if speed > &p {
                    return speed.clone();
                }
            }
            return p;
        });
    }
    if game_state.pipes.len() >= 4 {
        game_state.pipes[3].speed = game_state.upgrades.iter().fold(1u64, |p, upg| {
            if let Upgrade::Pipe4Speed(speed) = upg {
                if speed > &p {
                    return speed.clone();
                }
            }
            return p;
        });
    }
}

fn pipe_shape_drop_tick(game_state: &mut GameState, dt: u64) {
    for i in 0..game_state.pipes.len() {
        let mut drop_ct = 0;
        {
            let pipe = &mut game_state.pipes[i];
            pipe.last_drop += dt;
            if pipe.last_drop > drop_delay(pipe) {
                if pipe.last_drop == 0 {
                    drop_ct = 1;
                } else {
                    drop_ct = pipe.last_drop / drop_delay(pipe);
                }
            }
        }
        if drop_ct > 0 {
            drop_shapes(game_state, i as i32, drop_ct as u32);
            game_state.pipes[i].last_drop -= drop_ct * drop_delay(&game_state.pipes[i]);
        }
    }
}

fn sigmoid(x: f64) -> f64 {
    1f64 / (1f64 + E.powf(-x as f32) as f64)
}

fn drop_delay(pipe: &GamePipe) -> u64 {
    let per_sec;
    unsafe {
        per_sec = log2l(pipe.speed as f64)/log2l(1.2)*0.05*pipe.speed as f64 + 0.2;
    }
    let delay = 1000f64/per_sec;
    delay as u64
}

fn drop_shapes(state: &mut GameState, i: i32, drop_ct: u32) {
    let unlocked_colors: Vec<Color> = unlocked_colors(state, i);
    let pipe = &state.pipes[i as usize];
    for _ in 0..drop_ct {
        for color in &unlocked_colors {
            let pressure = (pipe.speed*unlocked_colors.len() as u64) as f64 * 0.03;
            let x_factor = rand::thread_rng().gen_range(-1f64..1f64);
            let x_offset_vel = (sigmoid(x_factor*5f64)*2f64 - 1f64) * 50f64;
            let x = 30 + 120*i + 33;
            let y = 30 + 10*i + 180;
            state.shapes.push(Shape { 
                x: x as f64 + rand::thread_rng().gen_range(-1f64..1f64)*10f64, 
                y: y as f64, 
                vel_x: x_offset_vel * pressure, 
                vel_y: 300f64, 
                rot: rand::thread_rng().gen_range(0f32..PI*360f32), 
                rot_vel: 0.1f32, 
                sides: pipe.sides, 
                color: color.clone(),
                laser_cuts: 0
            });
        }
    }
}

fn unlocked_colors(state: &mut GameState, i: i32) -> Vec<Color> {
    let mut result = vec![Color::GRAY];
    for upgrade in &state.upgrades {
        if i == 0 {
            if let Upgrade::Pipe1Color(c) = upgrade {
                result.push(c.clone());
            }
        } else if i == 1 {
            if let Upgrade::Pipe2Color(c) = upgrade {
                result.push(c.clone());
            }
        } else if i == 2 {
            if let Upgrade::Pipe3Color(c) = upgrade {
                result.push(c.clone());
            }
        } else if i == 3 { 
            if let Upgrade::Pipe4Color(c) = upgrade {
                result.push(c.clone());
            }
        }
    }
    return result;
}