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
        match rng.gen_range(0..=4) {
            0 => Gun::Sniper,
            1 => Gun::Shotgun,
            2 => Gun::SubMachineGun,
            3 => Gun::AssaultRifle,
            4 => Gun::MachineGun,
            _ => unreachable!(),
        }
    }
}

impl Gun {
    pub fn to_held_weapon(&self) -> HeldWeapon {
        HeldWeapon {
            gun: *self,
            ammo: self.max_ammo(),
        }
    }

    pub fn range(&self) -> f32 {
        match self {
            Gun::Pistol => 10.0,
            Gun::MachineGun => 10.0,
            Gun::Sniper => 10.0,
            Gun::Shotgun => 10.0,
            Gun::SubMachineGun => 10.0,
            Gun::AssaultRifle => 10.0,
        }
    }

    pub fn damage(&self) -> f32 {
        match self {
            Gun::Pistol => 10.0,
            Gun::MachineGun => 7.0,
            Gun::Sniper => 40.0,
            Gun::Shotgun => 120.0,
            Gun::SubMachineGun => 6.0,
            Gun::AssaultRifle => 15.0,
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

    pub fn dmg_drop_off(&self) -> f32 {
        match self {
            Gun::Pistol => 0.8,
            Gun::MachineGun => 0.8,
            Gun::Sniper => 5.0,
            Gun::Shotgun => 0.3,
            Gun::SubMachineGun => 0.7,
            Gun::AssaultRifle => 0.8,
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

    pub fn max_ammo(&self) -> usize {
        match self {
            Gun::Pistol => 0,
            Gun::MachineGun => 50,
            Gun::Sniper => 5,
            Gun::Shotgun => 2,
            Gun::SubMachineGun => 36,
            Gun::AssaultRifle => 26,
        }
    }

    pub fn spread(&self) -> Option<f32> {
        match self {
            Gun::MachineGun => Some(f32::to_radians(2.5)),
            Gun::Shotgun => Some(f32::to_radians(5.0)),
            Gun::SubMachineGun => Some(f32::to_radians(3.0)),
            Gun::AssaultRifle => Some(f32::to_radians(2.0)),
            _ => None,
        }
    }

    pub fn pellets(&self) -> usize {
        match self {
            Gun::Shotgun => 16,
            _ => 1,
        }
    }

    pub fn damage_with_drop_off(&self, distance:f32) -> f32 {      
        let dmg_per_pellet = self.damage() / self.pellets() as f32;




        lerp(dmg_per_pellet, dmg_per_pellet * self.dmg_drop_off(), distance)
    }
}

impl fmt::Display for Gun {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gun::Pistol => write!(f, "Glock 19"),
            Gun::MachineGun => write!(f, "M2 Browning"),
            Gun::Sniper => write!(f, "Barrett m82A1"),
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

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}