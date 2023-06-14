use std::collections::{HashMap, HashSet};

use crate::{
    moves::Move,
    player::{Player, PlayerId},
};

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

    pub fn start_game(&mut self) {
        self.game_state = GameState::Playing;
    }

    pub fn process_turn(&mut self, moves: &HashMap<PlayerId, Move>) {
        for (player, player_move) in moves {
            self.players
                .get_mut(player)
                .unwrap()
                .move_completed(*player_move);
        }
        let players: &GamePlayers = &self.players;
        // Make list of players who should die this round
        let players_to_kill: HashSet<PlayerId> = moves
            .iter()
            .map(|(player, player_move)| result_of_player_move(player, player_move, moves))
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
        for player_id in players_to_kill.iter() {
            if let Some(player) = self.players.get_mut(player_id) {
                player.kill();
            }
        }

        // Check for a winner
        if let Some(winner) = get_winner(&self.players) {
            self.game_state = GameState::Ended { winner };
        }
    }
}

enum MoveResult {
    NoKill,
    Kill(PlayerId),
    AllKill { attacker: PlayerId },
}

fn result_of_player_move(
    player: &PlayerId,
    player_move: &Move,
    moves: &HashMap<PlayerId, Move>,
) -> MoveResult {
    use crate::moves::Move::*;
    use MoveResult::*;

    match player_move {
        Kamehameha { target } => {
            let targets_move = moves.get(target).unwrap();
            match targets_move {
                Charge => Kill(*target),
                Kamehameha { target } => {
                    if *player == *target {
                        NoKill
                    } else {
                        Kill(*target)
                    }
                }
                SuperSaiyan => Kill(*target),
                Reflect => Kill(*player),
                _ => NoKill,
            }
        }

        Disk { target } => {
            let targets_move = moves.get(target).unwrap();
            match targets_move {
                Charge => Kill(*target),
                Kamehameha { target: _ } => Kill(*target),
                Disk { target } => {
                    if *player == *target {
                        NoKill
                    } else {
                        Kill(*target)
                    }
                }
                SuperSaiyan => Kill(*target),
                Reflect => Kill(*player),
                _ => NoKill,
            }
        }

        SpecialBeam { target } => {
            let targets_move = moves.get(target).unwrap();
            match targets_move {
                Charge => Kill(*target),
                Block => Kill(*target),
                Kamehameha { target: _ } => Kill(*target),
                Disk { target: _ } => Kill(*target),
                SuperSaiyan => Kill(*target),
                Reflect => Kill(*player),
                SpecialBeam { target } => {
                    if *player == *target {
                        NoKill
                    } else {
                        Kill(*target)
                    }
                }
                _ => NoKill,
            }
        }

        SpiritBomb => AllKill { attacker: *player },

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

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;
    use crate::{
        moves::Move,
        player::{Player, PlayerId},
    };
    #[test]
    fn two_players_one_killed_by_kamehameha() {
        let john = Player::new(1, "John".to_string());
        let mark = Player::new(2, "Mark".to_string());
        let mut game = Game::new(1234)
            .add_player(john.clone())
            .add_player(mark.clone());
        let turn1 = HashMap::from([(john.id, Move::Charge), (mark.id, Move::Charge)]);
        game.process_turn(&turn1);
        let turn2 = HashMap::from([
            (john.id, Move::Kamehameha { target: mark.id }),
            (mark.id, Move::Charge),
        ]);
        game.process_turn(&turn2);
        assert_eq!(game.game_state, GameState::Ended { winner: john.id });
    }
}
