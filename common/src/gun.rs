use serde::{Deserialize, Serialize};
use std::{fmt, time::Duration};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::ecs::components::HeldWeapon;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Gun {
    Pistol,
    MachineGun,
}

impl Distribution<Gun> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gun {
        match rng.gen_range(0..=1) {
            0 => Gun::Pistol,
            _ => Gun::MachineGun,
        }
    }
}

impl Gun {
    pub fn to_held_weapon(&self) -> HeldWeapon {
        HeldWeapon {
            gun: *self,
            ammo: self.get_max_ammo(),
        }
    }

    pub fn range(&self) -> f32 {
        match self {
            Gun::Pistol => 8.0,
            Gun::MachineGun => 10.0,
        }
    }

    pub fn damage(&self) -> i8 {
        match self {
            Gun::Pistol => 10,
            Gun::MachineGun => 7,
        }
    }

    pub fn bullet_speed(&self) -> f32 {
        10.0
    }

    pub fn recharge(&self) -> Duration {
        match self {
            Gun::Pistol => Duration::from_secs_f32(0.5),
            Gun::MachineGun => Duration::from_secs_f32(0.1),
        }
    }

    pub fn get_max_ammo(&self) -> usize {
        match self {
            Gun::Pistol => 25,
            Gun::MachineGun => 50,
        }
    }
}

impl fmt::Display for Gun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gun::Pistol => write!(f, "Glock 19"),
            Gun::MachineGun => write!(f, "M2 Browning"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pistol_range() {
        let pistol = Gun::Pistol;
        assert_eq!(pistol.range(), 3.0);
    }
    #[test]
    fn test_machine_gun_recharge() {
        let machine_gun = Gun::MachineGun;
        assert_eq!(machine_gun.recharge(), Duration::new(0, 100_000_000));
    }
}
