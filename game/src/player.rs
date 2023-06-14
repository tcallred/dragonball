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
    pub nickname: String,
    charges: Charges,
    state: PlayerState,
}

impl Player {
    pub fn new(id: PlayerId, nickname: String) -> Self {
        Self {
            id,
            nickname,
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
    pub fn move_completed(&mut self, choice: Move) {
        match choice {
            Move::Charge => {
                self.charge();
            }
            Move::SuperSaiyan => {
                self.state = PlayerState::SuperSaiyan;
            }
            _ => (),
        }
        self.deduct_charges(choice.cost());
    }

    pub fn kill(&mut self) {
        match self.state {
            PlayerState::SuperSaiyan => {
                self.state = PlayerState::Alive;
            }
            PlayerState::Alive => {
                self.state = PlayerState::Dead;
                self.charges = 0;
            }
            _ => (),
        }
    }

    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    fn charge(&mut self) {
        match self.state {
            PlayerState::Alive => {
                self.charges += 1;
            }
            PlayerState::SuperSaiyan => {
                self.charges += 2;
            }
            _ => (),
        }
    }

    fn has_charges_for(&self, choice: Move) -> bool {
        self.charges >= choice.cost()
    }

    fn deduct_charges(&mut self, ammount: Charges) {
        self.charges -= ammount;
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    #[test]
    fn can_do_with_n_charges() {
        let mut player = Player::new(1, "Jonny".to_string());

        assert_eq!(player.can_do_move(Move::Kamehameha { target: 2 }), false);

        player.move_completed(Move::Charge); // charges : 1

        assert_eq!(player.can_do_move(Move::Kamehameha { target: 2 }), true);

        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 3

        assert_eq!(player.can_do_move(Move::SuperSaiyan), true);
        assert_eq!(player.can_do_move(Move::SpecialBeam { target: 2 }), false);
    }

    #[test]
    fn dead_cant_move() {
        let mut player = Player::new(1, "Jonny".to_string());

        player.move_completed(Move::Charge); // charges : 1
        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 3

        player.kill();

        assert_eq!(player.state, PlayerState::Dead);

        assert_eq!(player.can_do_move(Move::Kamehameha { target: 2 }), false);
        assert_eq!(player.can_do_move(Move::Charge), false);
        assert_eq!(player.can_do_move(Move::Block), false);
    }

    #[test]
    fn super_saiyan_charges_double() {
        let mut player = Player::new(1, "Jonny".to_string());

        player.move_completed(Move::Charge); // charges : 1
        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 3

        assert_eq!(player.charges, 3);

        player.move_completed(Move::SuperSaiyan);

        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 6

        assert_eq!(player.charges, 6);
    }

    #[test]
    fn super_saiyan_has_extra_life() {
        let mut player = Player::new(1, "Jonny".to_string());

        player.move_completed(Move::Charge); // charges : 1
        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 3

        player.move_completed(Move::SuperSaiyan);

        assert_eq!(player.state, PlayerState::SuperSaiyan);

        player.kill();

        assert_eq!(player.state, PlayerState::Alive);
    }

    #[test]
    fn dead_players_have_no_charges() {
        let mut player = Player::new(1, "Jonny".to_string());

        player.move_completed(Move::Charge); // charges : 1

        assert_eq!(player.state, PlayerState::Alive);
        assert_eq!(player.charges, 1);

        player.kill();

        assert_eq!(player.state, PlayerState::Dead);
        assert_eq!(player.charges, 0);
    }

    #[test]
    fn deducts_correct_ammount() {
        let mut player = Player::new(1, "Jonny".to_string());

        player.move_completed(Move::Charge); // charges : 1
        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 3
        player.move_completed(Move::Charge); // charges : 4
        player.move_completed(Move::Charge); // charges : 5

        assert_eq!(player.charges, 5);

        player.move_completed(Move::SpecialBeam { target: 2 }); // charges : 0

        assert_eq!(player.charges, 0);
    }

    #[test]
    fn deducts_correct_ammount_with_super() {
        let mut player = Player::new(1, "Jonny".to_string());

        player.move_completed(Move::Charge); // charges : 1
        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 3

        assert_eq!(player.charges, 3);

        player.move_completed(Move::SuperSaiyan); // charges : 0

        assert_eq!(player.charges, 0);

        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 2
        player.move_completed(Move::Charge); // charges : 6

        player.move_completed(Move::SpecialBeam { target: 2 }); // charges : 1

        assert_eq!(player.charges, 1);
    }
}
