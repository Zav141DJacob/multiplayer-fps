use std::time::Duration;
use serde::{Serialize, Deserialize};

use rand::Rng;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Gun {
    Pistol,
    MachineGun,
}
impl Gun {
    pub fn range(&self) -> f32 {
        match self {
            Gun::Pistol => 3.0,
            Gun::MachineGun => 5.0,
        }
    }
    pub fn recharge(&self) -> Duration {
        match self {
            Gun::Pistol => Duration::new(1,0),
            Gun::MachineGun => Duration::new(0,100_000_000),
        }
    }
    pub fn get_random_gun() -> Gun {
        let index = rand::thread_rng().gen_range(0..=1);
        match index {
            0 => Gun::Pistol,
            _ => Gun::MachineGun,
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
        assert_eq!(machine_gun.recharge(), Duration::new(0,100_000_000));
    }
}