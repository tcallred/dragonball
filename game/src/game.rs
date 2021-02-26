use std::collections::{HashMap, HashSet};

use crate::{
    moves::PlayerMove,
    player::{Player, PlayerId},
};

pub type GameId = u32;
type GamePlayers = HashMap<PlayerId, Player>;

enum GameState {
    Setup,
    Playing,
    Ended {winner: PlayerId},
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

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }

    pub fn start_game(&mut self) {
        self.game_state = GameState::Playing;
    }

    pub fn process_round(&mut self, moves: &HashMap<PlayerId, PlayerMove>) {

        // TODO: Handle spirit bomb

        // Make list of players who should die this round
        let players_to_kill: HashSet<PlayerId> = moves
            .iter()
            .map(|(_, player_move)| player_killed_by(player_move, moves))
            .filter_map(|player_id| player_id)
            .collect();

        // Kill players who should die
        for player_id in players_to_kill.iter() {
            if let Some(player) = self.players.get_mut(player_id){
                player.kill();
            }
        }

        // Check for a winner
        if let Some(winner) = get_winner(&self.players) {
            self.game_state = GameState::Ended{winner};
        }

    }
}

fn player_killed_by(
    player_move: &PlayerMove,
    moves: &HashMap<PlayerId, PlayerMove>,
) -> Option<PlayerId> {
    use crate::moves::Move::*;

    match player_move.choice {
        Kamehameha { target } => {
            let targets_move = moves.get(&target).unwrap().choice;
            match targets_move {
                Charge => Some(target),
                SuperSaiyan => Some(target),
                Reflect {
                    target: reflect_target,
                } => player_killed_by(
                    &PlayerMove::new(
                        target,
                        Kamehameha {
                            target: reflect_target,
                        },
                    ),
                    moves,
                ),
                _ => None,
            }
        }

        Disk { target } => {
            let targets_move = moves.get(&target).unwrap().choice;
            match targets_move {
                Charge => Some(target),
                Kamehameha { target: _ } => Some(target),
                SuperSaiyan => Some(target),
                Reflect {
                    target: reflect_target,
                } => player_killed_by(
                    &PlayerMove::new(
                        target,
                        Disk {
                            target: reflect_target,
                        },
                    ),
                    moves,
                ),
                _ => None,
            }
        }

        SpecialBeam { target } => {
            let targets_move = moves.get(&target).unwrap().choice;
            match targets_move {
                Charge => Some(target),
                Block => Some(target),
                Kamehameha{target: _} => Some(target),
                Disk{target:_} => Some(target),
                SuperSaiyan => Some(target),
                Reflect {
                    target: reflect_target,
                } => player_killed_by(
                    &PlayerMove::new(
                        target,
                        SpecialBeam {
                            target: reflect_target,
                        },
                    ),
                    moves,
                ),
                _ => None
            }
        }
        _ => None,
    }
}


fn get_winner(players: &GamePlayers) -> Option<PlayerId> {
    let alive: Vec<PlayerId> = players.iter()
        .map(|(_, player)| player)
        .filter(|player| !player.is_dead())
        .map(|player| player.id)
        .collect();

    if alive.len() == 1 {
        Some(alive[0])
    }
    else {
        None
    }
}