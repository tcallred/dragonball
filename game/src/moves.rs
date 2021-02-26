use crate::player::PlayerId;


pub type Charges = u8;

#[derive(Clone, Copy, PartialEq)]
pub enum Move {
    Charge,
    Block,
    Kamehameha{target: PlayerId},
    Disk{target: PlayerId},
    SuperSaiyan,
    Reflect{target: PlayerId},
    SpecialBeam{target: PlayerId},
    SpiritBomb
}

pub struct PlayerMove {
    pub player: PlayerId,
    pub choice: Move,
}

impl PlayerMove {
    pub fn new(player: PlayerId, choice: Move) -> Self {
        Self {
            player,
            choice
        }
    }
}

impl Move {
    pub fn cost(&self) -> Charges {
        use Move::*;

        // TODO Maybe use table lookup?
        match self {
            Charge => 0,
            Block => 0,
            Kamehameha{target: _} => 1,
            Disk{target: _} => 2,
            SuperSaiyan => 3,
            Reflect{target: _} => 4,
            SpecialBeam{target: _} => 5,
            SpiritBomb => 7
        }
    }
}
