use crate::moves::*;

pub type PlayerId = u32;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PlayerState {
    Alive,
    SuperSaiyan,
    Dead,
}

#[derive(Clone)]
pub struct Player {
    pub id: PlayerId,
    pub nickname: std::rc::Rc<str>,
    charges: Charges,
    state: PlayerState,
}

impl Player {
    pub fn new(id: PlayerId, nickname: &str) -> Self {
        Self {
            id,
            nickname: std::rc::Rc::from(nickname),
            charges: 0,
            state: PlayerState::Alive,
        }
    }

    pub fn can_do_move(&self, choice: Move) -> bool {
        match choice {
            Move::SuperSaiyan => self.state == PlayerState::Alive && self.has_charges_for(choice),
            _ => self.state != PlayerState::Dead && self.has_charges_for(choice),
        }
    }

    // Called after move has been processed
    pub fn move_completed(&self, choice: Move) -> Self {
        let after_deducted = self.deduct_charges(choice.cost());
        match choice {
            Move::Charge => self.charge(),
            Move::SuperSaiyan => Self {
                state: PlayerState::SuperSaiyan,
                ..after_deducted
            },
            _ => after_deducted,
        }
    }

    pub fn kill(&self) -> Self {
        let next_state = match self.state {
            PlayerState::SuperSaiyan => PlayerState::Alive,
            PlayerState::Alive => PlayerState::Dead,
            _ => self.state,
        };

        Self {
            state: next_state,
            charges: if next_state == PlayerState::Dead {
                0
            } else {
                self.charges
            },
            ..self.clone()
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    fn charge(&self) -> Self {
        let charge_ammount = match self.state {
            PlayerState::Alive => 1,
            PlayerState::SuperSaiyan => 2,
            _ => 0,
        };

        Self {
            charges: self.charges + charge_ammount,
            ..self.clone()
        }
    }

    fn has_charges_for(&self, choice: Move) -> bool {
        self.charges >= choice.cost()
    }

    fn deduct_charges(&self, ammount: Charges) -> Self {
        Self {
            charges: self.charges - ammount,
            ..self.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    #[test]
    fn can_do_with_n_charges() {
        let mut player = Player::new(1, "Jonny");

        assert_eq!(player.can_do_move(Move::Kamehameha { target: 2 }), false);

        player = player.move_completed(Move::Charge); // charges : 1

        assert_eq!(player.can_do_move(Move::Kamehameha { target: 2 }), true);

        player = player
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge); // charges : 3

        assert_eq!(player.can_do_move(Move::SuperSaiyan), true);
        assert_eq!(player.can_do_move(Move::SpecialBeam { target: 2 }), false);
    }

    #[test]
    fn dead_cant_move() {
        let mut player = Player::new(1, "Jonny");

        player = player
            .move_completed(Move::Charge) // charges : 1
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge) // charges : 3
            .kill();

        assert_eq!(player.state, PlayerState::Dead);

        assert_eq!(player.can_do_move(Move::Kamehameha { target: 2 }), false);
        assert_eq!(player.can_do_move(Move::Charge), false);
        assert_eq!(player.can_do_move(Move::Block), false);
    }

    #[test]
    fn super_saiyan_charges_double() {
        let mut player = Player::new(1, "Jonny");

        player = player
            .move_completed(Move::Charge) // charges : 1
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge); // charges : 3

        assert_eq!(player.charges, 3);

        player = player
            .move_completed(Move::SuperSaiyan)
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge); // charges : 6

        assert_eq!(player.charges, 6);
    }

    #[test]
    fn super_saiyan_has_extra_life() {
        let mut player = Player::new(1, "Jonny");

        player = player
            .move_completed(Move::Charge) // charges : 1
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge) // charges : 3
            .move_completed(Move::SuperSaiyan);

        assert_eq!(player.state, PlayerState::SuperSaiyan);

        player = player.kill();

        assert_eq!(player.state, PlayerState::Alive);
    }

    #[test]
    fn dead_players_have_no_charges() {
        let mut player = Player::new(1, "Jonny");

        player = player.move_completed(Move::Charge); // charges : 1

        assert_eq!(player.state, PlayerState::Alive);
        assert_eq!(player.charges, 1);

        player = player.kill();

        assert_eq!(player.state, PlayerState::Dead);
        assert_eq!(player.charges, 0);
    }

    #[test]
    fn deducts_correct_ammount() {
        let mut player = Player::new(1, "Jonny");

        player = player
            .move_completed(Move::Charge) // charges : 1
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge) // charges : 3
            .move_completed(Move::Charge) // charges : 4
            .move_completed(Move::Charge); // charges : 5

        assert_eq!(player.charges, 5);

        player = player.move_completed(Move::SpecialBeam { target: 2 }); // charges : 0

        assert_eq!(player.charges, 0);
    }

    #[test]
    fn deducts_correct_ammount_with_super() {
        let mut player = Player::new(1, "Jonny");

        player = player
            .move_completed(Move::Charge) // charges : 1
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge); // charges : 3

        assert_eq!(player.charges, 3);

        player = player.move_completed(Move::SuperSaiyan); // charges : 0

        assert_eq!(player.charges, 0);

        player = player
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge) // charges : 2
            .move_completed(Move::Charge) // charges : 6
            .move_completed(Move::SpecialBeam { target: 2 }); // charges : 1

        assert_eq!(player.charges, 1);
    }
}
