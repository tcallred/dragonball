use std::collections::HashMap;

use crate::player::Player;

pub type GameId = u32;

struct Game {
    pub id: GameId,
    players: HashMap<u32, Player>
}

impl Game {
    pub fn new(id: GameId) -> Self {
        Self {
            id,
            players: HashMap::new()
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.insert(player.id, player);
    }
}