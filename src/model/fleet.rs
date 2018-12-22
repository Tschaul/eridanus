use crate::model::base_types::Amount;
use crate::model::player::PlayerToken;
use crate::model::world::WorldKey;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct FleetKey(u8);

impl FleetKey {
    pub fn new(key_value: u8) -> Self {
        FleetKey(key_value)
    }
}

impl std::fmt::Display for FleetKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct Fleet {
    pub owner: Option<PlayerToken>,
    pub world: WorldKey,
    pub ships: Amount,
}