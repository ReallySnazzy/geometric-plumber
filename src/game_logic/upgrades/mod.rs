pub mod unlock_upgrades;
mod purchase;

use std::rc::Rc;

use num::BigInt;
use raylib::prelude::Color;

use self::{unlock_upgrades::{UpgradeRequirement, unlock_upgrades_tick}, purchase::upgrade_purchase_tick};

use super::state::GameState;

#[derive(PartialEq, Clone, Debug)]
pub enum Upgrade {
    Pipe1Speed(u64),
    Pipe1Color(Color),

    Pipe2,
    Pipe2Speed(u64),
    Pipe2Color(Color),

    Pipe3,
    Pipe3Speed(u64),
    Pipe3Color(Color),

    Pipe4,
    Pipe4Speed(u64),
    Pipe4Color(Color),

    ShapeLaser(u64),
}

#[derive(Clone)]
pub struct PurchasableUpgrade {
    pub upgrade: Upgrade,
    pub price: BigInt,
    pub description: String,
    requirements: Vec<Rc<dyn UpgradeRequirement>>,
}

pub fn upgrade_tick(state: &mut GameState) {
    unlock_upgrades_tick(state);
    upgrade_purchase_tick(state);
}
