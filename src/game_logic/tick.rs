use super::state::GameState;
use super::pipe::pipe_tick;
use super::shape::shape_tick;
use super::upgrades::upgrade_tick;

pub fn tick(state: &mut GameState) {
    let now = super::time::current_time();
    let mut dt = now - state.last_tick;
    if state.last_tick == 0 {
        dt = 1000 / 30
    }
    pipe_tick(state, dt);
    shape_tick(state, dt);
    upgrade_tick(state);
    state.last_tick = now;
}

