use crate::player::PlayerId;

pub type Charges = u8;

#[derive(Clone, Copy, PartialEq)]
pub enum Move {
    Charge,
    Block,
    Kamehameha { target: PlayerId },
    Disk { target: PlayerId },
    SuperSaiyan,
    Reflect,
    SpecialBeam { target: PlayerId },
    SpiritBomb,
}

impl Move {
    pub fn cost(&self) -> Charges {
        use Move::*;

        match self {
            Charge => 0,
            Block => 0,
            Kamehameha { target: _ } => 1,
            Disk { target: _ } => 2,
            SuperSaiyan => 3,
            Reflect => 4,
            SpecialBeam { target: _ } => 5,
            SpiritBomb => 7,
        }
    }
}
