use serde::{Deserialize, Serialize};
use std::{fmt, time::Duration};

use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::ecs::components::HeldWeapon;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Gun {
    Pistol,
    Sniper,
    Shotgun,
    SubMachineGun,
    AssaultRifle,
    MachineGun,
}

impl Distribution<Gun> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Gun {
        match rng.gen_range(0..=5) {
            0 => Gun::Pistol,
            1 => Gun::Sniper,
            2 => Gun::Shotgun,
            3 => Gun::SubMachineGun,
            4 => Gun::AssaultRifle,
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
            Gun::Sniper => 100.0,
            Gun::Shotgun => 5.0,
            Gun::SubMachineGun => 9.0,
            Gun::AssaultRifle => 15.0,
        }
    }

    pub fn damage(&self) -> i8 {
        match self {
            Gun::Pistol => 10,
            Gun::MachineGun => 7,
            Gun::Sniper => 100,
            Gun::Shotgun => 50,
            Gun::SubMachineGun => 6,
            Gun::AssaultRifle => 15,
        }
    }

    pub fn bullet_speed(&self) -> f32 {
        match self {
            Gun::Pistol => 10.0,
            Gun::MachineGun => 10.0,
            Gun::Sniper => 20.0,
            Gun::Shotgun => 8.0,
            Gun::SubMachineGun => 10.0,
            Gun::AssaultRifle => 12.0,
        }
    }

    pub fn recharge(&self) -> Duration {
        match self {
            Gun::Pistol => Duration::from_secs_f32(0.2),
            Gun::MachineGun => Duration::from_secs_f32(0.1),
            Gun::Sniper => Duration::from_secs_f32(1.5),
            Gun::Shotgun => Duration::from_secs_f32(0.2),
            Gun::SubMachineGun => Duration::from_secs_f32(0.05),
            Gun::AssaultRifle => Duration::from_secs_f32(0.15),
        }
    }

    pub fn get_max_ammo(&self) -> usize {
        match self {
            Gun::Pistol => 25,
            Gun::MachineGun => 50,
            Gun::Sniper => 5,
            Gun::Shotgun => 2,
            Gun::SubMachineGun => 36,
            Gun::AssaultRifle => 26,
        }
    }
}

impl fmt::Display for Gun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gun::Pistol => write!(f, "Glock 19"),
            Gun::MachineGun => write!(f, "M2 Browning"),
            Gun::Sniper => write!(f, "barrett m82A1"),
            Gun::Shotgun => write!(f, "Browning BSS"),
            Gun::SubMachineGun => write!(f, "KRISS Vector"),
            Gun::AssaultRifle => write!(f, "Remington ACR"),
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
