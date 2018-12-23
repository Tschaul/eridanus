use crate::model::base_types::Amount;
use crate::model::player::PlayerToken;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]
pub struct WorldKey(u8);

impl WorldKey {
    pub fn new(key_value: u8) -> Self {
        WorldKey(key_value)
    }
}

impl std::fmt::Display for WorldKey {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let WorldKey(key_value) = self;
        write!(f, "{}", key_value)
    }
}

#[derive(Debug, Clone)]
pub struct World {
    pub owner: Option<PlayerToken>,
    pub industry: Amount,
    pub metal: Amount,
    pub mines: Amount,
    pub population: Amount,
    pub limit: Amount,
    pub turns: Amount,
    pub i_ships: Amount,
    pub p_ships: Amount,
}

impl std::fmt::Display for World {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, 
            "[{owner}] (Industry={industry}, Metal={metal}, Mines={mines}, Population={population}, Limit={limit}, Turns={turns}, I-Ships={i_ships}, P-Ships={p_ships})", 
            owner = match &self.owner {
                Some(player) => player.to_string(),
                None => String::from("")
            },
            industry = self.industry,
            metal = self.metal,
            mines = self.mines,
            population = self.population,
            limit = self.limit,
            turns = self.turns,
            i_ships = self.i_ships,
            p_ships = self.p_ships,
        )
    }
}