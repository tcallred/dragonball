#![allow(dead_code)]
use crate::{
    moves::Move,
    player::{Player, PlayerId},
};
use std::collections::{HashMap, HashSet};
pub type GameId = u32;
type GamePlayers = HashMap<PlayerId, Player>;

#[derive(PartialEq, Debug)]
enum GameState {
    Setup,
    Playing,
    Ended { winner: PlayerId },
}

struct Game {
    pub id: GameId,
    players: GamePlayers,
    game_state: GameState,
}

enum MoveResult {
    NoKill,
    Kill(PlayerId),
    AllKill { attacker: PlayerId },
}

impl Game {
    pub fn new(id: GameId) -> Self {
        Self {
            id,
            players: HashMap::new(),
            game_state: GameState::Setup,
        }
    }

    pub fn add_player(mut self, player: Player) -> Self {
        self.players.insert(player.id, player);
        self
    }

    pub fn start_game(mut self) -> Self {
        self.game_state = GameState::Playing;
        self
    }

    pub fn process_turn(mut self, moves: &HashMap<PlayerId, Move>) -> Self {
        let players: &GamePlayers = &self.players;
        // Make list of players who should die this round
        let players_to_kill: HashSet<PlayerId> = moves
            .iter()
            .map(|(player, player_move)| Game::result_of_player_move(*player, *player_move, moves))
            .fold(
                HashSet::new(),
                |mut killed_set, move_result| match move_result {
                    MoveResult::NoKill => killed_set,
                    MoveResult::Kill(player) => {
                        killed_set.insert(player);
                        killed_set
                    }
                    MoveResult::AllKill { attacker } => {
                        for player in players.keys() {
                            if *player != attacker {
                                killed_set.insert(*player);
                            }
                        }
                        killed_set
                    }
                },
            );

        // Kill players who should die
        self.players = self.players
            .into_iter()
            .map(|(key, player)| {
                if players_to_kill.contains(&player.id) {
                    (key, player.kill())
                } else {
                    (key, player)
                }
            })
            .collect();

        // Check for a winner
        if let Some(winner) = Game::get_winner(&self.players) {
            self.game_state = GameState::Ended { winner };
        }

        self
    }

    fn result_of_player_move(
        player: PlayerId,
        player_move: Move,
        moves: &HashMap<PlayerId, Move>,
    ) -> MoveResult {
        use crate::moves::Move::*;
        use MoveResult::*;

        match player_move {
            Kamehameha { target } => {
                let targets_move = *moves.get(&target).unwrap();
                match targets_move {
                    Charge => Kill(target),
                    Kamehameha { target } => {
                        if player == target {
                            NoKill
                        } else {
                            Kill(target)
                        }
                    }
                    SuperSaiyan => Kill(target),
                    Reflect => Kill(player),
                    _ => NoKill,
                }
            }

            Disk { target } => {
                let targets_move = *moves.get(&target).unwrap();
                match targets_move {
                    Charge => Kill(target),
                    Kamehameha { target: _ } => Kill(target),
                    Disk { target } => {
                        if player == target {
                            NoKill
                        } else {
                            Kill(target)
                        }
                    }
                    SuperSaiyan => Kill(target),
                    Reflect => Kill(player),
                    _ => NoKill,
                }
            }

            SpecialBeam { target } => {
                let targets_move = *moves.get(&target).unwrap();
                match targets_move {
                    Charge => Kill(target),
                    Block => Kill(target),
                    Kamehameha { target: _ } => Kill(target),
                    Disk { target: _ } => Kill(target),
                    SuperSaiyan => Kill(target),
                    Reflect => Kill(player),
                    SpecialBeam { target } => {
                        if player == target {
                            NoKill
                        } else {
                            Kill(target)
                        }
                    }
                    _ => NoKill,
                }
            }

            SpiritBomb => AllKill { attacker: player },

            _ => NoKill,
        }
    }

    fn get_winner(players: &GamePlayers) -> Option<PlayerId> {
        let alive: Vec<PlayerId> = players
            .iter()
            .map(|(_, player)| player)
            .filter(|player| !player.is_dead())
            .map(|player| player.id)
            .collect();

        if alive.len() == 1 {
            Some(alive[0])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    use crate::{moves::Move, player::Player};
    #[test]
    fn two_players_one_killed_by_kamehameha() {
        let john = Player::new(1, "John");
        let mark = Player::new(2, "Mark");
        let game = Game::new(1234)
            .add_player(john.clone())
            .add_player(mark.clone())
            .start_game();
        let turn1 = HashMap::from([(john.id, Move::Charge), (mark.id, Move::Charge)]);
        let turn2 = HashMap::from([
            (john.id, Move::Kamehameha { target: mark.id }),
            (mark.id, Move::Charge),
        ]);
        let final_game = game.process_turn(&turn1).process_turn(&turn2);
        assert_eq!(final_game.game_state, GameState::Ended { winner: john.id });
    }
}
