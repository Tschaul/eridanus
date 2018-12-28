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
        let FleetKey(value) = self;
        write!(f, "{}", value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fleet {
    pub owner: Option<PlayerToken>,
    pub world: WorldKey,
    pub ships: Amount,
}

impl Fleet {
    pub fn parse_print_out(fleet_print_out: &str, world_key: WorldKey) -> Result<Self, String> {
        let mut fleet = Fleet {
            owner: None,
            world: world_key,
            ships: Amount::new(0)
        };
        
        let split_equal: Vec<&str> = fleet_print_out.split('=').collect(); 
        let owner_key_value = String::from(split_equal[0].trim_matches(|c| c == '[' || c == ']' ));

        if !owner_key_value.is_empty() {
            fleet.owner = Some(PlayerToken::new(owner_key_value));
        }
        let ships: u8 = match split_equal[1].parse() {
            Ok(value) => value,
            Err(_) => return Err(format!("Error parsing fleet ships: {}", fleet_print_out))
        };

        fleet.ships = Amount::new(ships);

        Ok(fleet)
    }
}

impl std::fmt::Display for Fleet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
            "[{}]={}", 
            match &self.owner {
                Some(player) => player.to_string(),
                None => String::from("")
            }, 
            self.ships
        )
    }
}