use crate::model::base_types::Amount;
use crate::model::player::PlayerToken;

pub struct WorldKey {
    keyValue: u8,
}

impl WorldKey {
    pub fn new(keyValue: u8) -> Self {
        WorldKey { keyValue: keyValue }
    }
}

pub struct World {
    pub key: WorldKey,
    pub player: PlayerToken,
    pub industry: Amount,
    pub metal: Amount,
    pub mines: Amount,
    pub population: Amount,
    pub limit: Amount,
    pub turns: Amount,
    pub i_ships: Amount,
    pub p_ships: Amount,
}