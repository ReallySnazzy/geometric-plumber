use crate::{drawing::WIDTH, game_logic::state::GameState};

use super::PurchasableUpgrade;

pub fn upgrade_purchase_tick(state: &mut GameState) {
    if state.mouse_click_x != 0 || state.mouse_click_y != 0 {
        let x = state.mouse_click_x;
        let y = state.mouse_click_y;
        state.mouse_click_x = 0;
        state.mouse_click_y = 0;
        on_upgrade_clicked(state, x, y);
    }
}

fn on_upgrade_clicked(state: &mut GameState, x: i32, y: i32) {
    if let Some(upgrade) = upgrade_from_coordinates(state, x, y) {
        if upgrade.price <= state.score {
            state.score -= upgrade.price;
            state.upgrades.push(upgrade.upgrade);
        }
    }
}

fn upgrade_from_coordinates(state: &mut GameState, x: i32, y: i32) -> Option<PurchasableUpgrade> {
    for (i, upgrade) in state.available_upgrades.iter().enumerate() {
        let button_x = WIDTH - 50 - 32 + 10;
        let button_y: i32 = 32 + 24 + 12 +  32 + 12 + i as i32 * (36 + 12);
        let width = 50;
        let height = 36;
        if (button_x..(button_x+width)).contains(&x) && (button_y..(button_y+height)).contains(&y) {
            return Some(upgrade.clone());
        }
    }
    None
}
