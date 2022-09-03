use std::{rc::Rc};

use num::{BigInt, FromPrimitive};
use once_cell::unsync::Lazy;
use raylib::prelude::Color;

use crate::game_logic::state::GameState;

use super::{PurchasableUpgrade, Upgrade};

pub trait UpgradeRequirement {
    fn unlockable(&self, state: &mut GameState) -> bool;

    fn rc(self) -> Rc<Self> where Self: Sized {
        Rc::from(self)
    }
}

pub struct HasPipe {
    pub pipe: u32
}

impl HasPipe {
    pub fn new(pipe: u32) -> HasPipe {
        HasPipe { pipe }
    }
}

impl UpgradeRequirement for HasPipe {
    fn unlockable(&self, state: &mut GameState) -> bool {
        state.pipes.len() >= self.pipe as usize
    }
}

pub struct HasUpgrade {
    pub required_upgrade: Upgrade
}

impl HasUpgrade {
    pub fn new(upgrade: Upgrade) -> HasUpgrade {
        HasUpgrade { required_upgrade: upgrade }
    }
}

impl UpgradeRequirement for HasUpgrade {
    fn unlockable(&self, state: &mut GameState) -> bool {
        state.upgrades.contains(&self.required_upgrade)
    }
}

pub fn unlock_upgrades_tick(state: &mut GameState) {
    state.available_upgrades = ALL_UPGRADES_MEMO
        .iter()
        .cloned()
        .filter(
            |u| !state.upgrades.contains(&u.upgrade) 
                && u.requirements.iter().all(|r| r.unlockable(state)))     
        .collect();
    state.available_upgrades
        .sort_by(|a, b| a.price.partial_cmp(&b.price).unwrap());
}

const ALL_UPGRADES_MEMO: Lazy<Vec<PurchasableUpgrade>> = Lazy::new(|| {
    let mut upgrades = vec![
        PurchasableUpgrade {
            upgrade: Upgrade::Pipe2,
            price: BigInt::from_u64(10_000).unwrap(),
            description: "Square Pipe".to_owned(),
            requirements: vec![HasUpgrade::new(Upgrade::Pipe1Speed(3)).rc()]
        },
        PurchasableUpgrade {
            upgrade: Upgrade::Pipe3,
            price: BigInt::from_u64(10_000_000).unwrap(),
            description: "Pentagon Pipe".to_owned(),
            requirements: vec![HasPipe::new(2).rc()]
        },
        PurchasableUpgrade {
            upgrade: Upgrade::Pipe4,
            price: BigInt::from_u64(10_000_000_000).unwrap(),
            description: "Hexagon Pipe".to_owned(),
            requirements: vec![HasPipe::new(3).rc()]
        },
        PurchasableUpgrade {
            upgrade: Upgrade::ShapeLaser(1),
            price: BigInt::from_u64(500_000_000_000_000_000).unwrap(),
            description: "Shape Cutting Laser".to_owned(),
            requirements: vec![HasPipe::new(3).rc()]
        }
    ];

    let shape_colors: Vec<(Color, &'static str)> = vec![
        (Color::RED, "Red"),
        (Color::GREEN, "Green"),
        (Color::BLUE, "Blue"),
        (Color::YELLOW, "Yellow"),
        (Color::PURPLE, "Purple")
    ];

    for (i, c) in shape_colors.iter().enumerate() {
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe1Color(c.0.clone()),
            price: BigInt::from_u64(100u64.pow(i as u32 + 1) + 700).unwrap(),
            description: format!("Triangle Pipe {}", c.1),
            requirements: match i {
                0 => vec![],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe1Color(shape_colors[n - 1].0.clone())
                    ).rc()
                ]
            }
        });
    }

    for (i, c) in shape_colors.iter().enumerate() {
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe2Color(c.0.clone()),
            price: BigInt::from_u64(50_000).unwrap()
                * BigInt::from_u64(100u64.pow(i as u32 + 1) + 700).unwrap(),
            description: format!("Square Pipe {}", c.1),
            requirements: match i {
                0 => vec![HasPipe::new(2).rc()],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe2Color(shape_colors[n - 1].0.clone())
                    ).rc()
                ]
            }
        });
    }

    for (i, c) in shape_colors.iter().enumerate() {
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe3Color(c.0.clone()),
            price: BigInt::from_u64(1_000_000).unwrap() 
                * BigInt::from_u64(100u64.pow(i as u32 + 1) + 700).unwrap(),
            description: format!("Pentagon Pipe {}", c.1),
            requirements: match i {
                0 => vec![HasPipe::new(3).rc()],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe3Color(shape_colors[n - 1].0.clone())
                    ).rc()]
            }
        });
    }

    for (i, c) in shape_colors.iter().enumerate() {
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe4Color(c.0.clone()),
            price: BigInt::from_u64(100_000_000).unwrap() 
                * BigInt::from_u64(100u64.pow(i as u32 + 1) + 700).unwrap(),
            description: format!("Hexagon Pipe {}", c.1),
            requirements: match i {
                0 => vec![HasPipe::new(4).rc()],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe3Color(shape_colors[n - 1].0.clone())
                    ).rc()]
            }
        });
    }

    for i in 1..=40 {
        let if64 = i as f64;
        let cost = 3.2f64.powf(if64)*12f64 + 40f64 + 10f64*if64*if64;
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe1Speed(i),
            price: BigInt::from_f64(cost.floor()).unwrap(),
            description: "Triangle Pipe Speed".to_owned(),
            requirements: match i {
                1 => vec![],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe1Speed(n - 1)
                    ).rc()]
            }
        });
    }

    for i in 1..=40 {
        let if64 = i as f64;
        let cost = 4_000_000f64*3.2f64.powf(if64)*12f64 + 40f64 + 10f64*if64*if64;
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe2Speed(i),
            price: BigInt::from_f64(cost.floor()).unwrap(),
            description: "Square Pipe Speed".to_owned(),
            requirements: match i {
                1 => vec![HasPipe::new(2).rc()],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe2Speed(n - 1)
                    ).rc()
                ]
            }
        });
    }

    for i in 1..=40 {
        let if64 = i as f64;
        let cost = 50_000_000f64*3.2f64.powf(if64)*12f64 + 40f64 + 10f64*if64*if64;
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe3Speed(i),
            price: BigInt::from_f64(cost.floor()).unwrap(),
            description: "Pentagon Pipe Speed".to_owned(),
            requirements: match i {
                1 => vec![HasPipe::new(3).rc()],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe3Speed(n - 1)
                    ).rc()
                ]
            }
        });
    }

    for i in 1..=40 {
        let if64 = i as f64;
        let cost = 200_000_000f64*3.2f64.powf(if64)*12f64 + 40f64 + 10f64*if64*if64;
        upgrades.push(PurchasableUpgrade {
            upgrade: Upgrade::Pipe4Speed(i),
            price: BigInt::from_f64(cost.floor()).unwrap(),
            description: "Hexagon Pipe Speed".to_owned(),
            requirements: match i {
                1 => vec![HasPipe::new(4).rc()],
                n => vec![
                    HasUpgrade::new(
                        Upgrade::Pipe4Speed(n - 1)
                    ).rc()
                ]
            }
        });
    }

    return upgrades;
});
