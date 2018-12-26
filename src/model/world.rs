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

impl World {
    pub fn parse_print_out(world_print_out: &str) -> Result<Self, String> {
        let mut world = World {
            owner: None,
            industry: Amount::new(0),
            metal: Amount::new(0),
            mines: Amount::new(0),
            population: Amount::new(0),
            limit: Amount::new(0),
            turns: Amount::new(0),
            i_ships: Amount::new(0),
            p_ships: Amount::new(0),
        };

        let owner_split: Vec<&str> = world_print_out.splitn(2, ' ').collect();

        let owner_key_value: String = String::from(owner_split[0].trim_matches(|c| c == '[' || c == ']' ));

        if !owner_key_value.is_empty() {
            let owner_key = PlayerToken::new(owner_key_value);
            world.owner = Some(owner_key);
        }

        let world_parts: Vec<&str> = owner_split[1].trim_matches(|c| c == '(' || c == ')' ).split(',').collect();
        
        for world_part in world_parts {
            let split: Vec<&str> = world_part.split('=').collect();
            match split[0].trim() as &str {
                "Industry" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse industry: {}", &split[1]))
                    };
                    world.industry = Amount::new(value)
                },
                "Metal" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse metal: {}", &split[1]))
                    };
                    world.metal = Amount::new(value)
                },
                "Mines" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse mines: {}", &split[1]))
                    };
                    world.mines = Amount::new(value)
                },
                "Population" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse population: {}", &split[1]))
                    };
                    world.population = Amount::new(value)
                },
                "Limit" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse limit: {}", &split[1]))
                    };
                    world.limit = Amount::new(value)
                },
                "Turns" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse turns: {}", &split[1]))
                    };
                    world.turns = Amount::new(value)
                },
                "I-Ships" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse i_ships: {}", &split[1]))
                    };
                    world.i_ships = Amount::new(value)
                },
                "P-Ships" => {
                    let value: u8 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse p_ships: {}", &split[1]))
                    };
                    world.p_ships = Amount::new(value)
                },
                _ => {}
            }
        }

        Ok(world)
    }

    
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