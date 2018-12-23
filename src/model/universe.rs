extern crate im_rc;

use im_rc::HashMap;
use im_rc::HashSet;

use crate::model::fleet::Fleet;
use crate::model::fleet::FleetKey;
use crate::model::world::World;
use crate::model::world::WorldKey;
use crate::model::player::PlayerToken;
use crate::model::gate::Gate;

pub struct Universe {
    fleets: HashMap<FleetKey, Fleet>,
    worlds: HashMap<WorldKey, World>,
    gates: HashSet<Gate>
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            fleets: HashMap::new(),
            worlds: HashMap::new(),
            gates: HashSet::new()
        }
    }

    pub fn with_updated_fleet(&self, fleet_key: &FleetKey, fleet: Fleet) -> Self {
        Universe {
            fleets: self.fleets.update(fleet_key.clone(), fleet),
            worlds: self.worlds.clone(),
            gates: self.gates.clone(),
        }
    } 

    pub fn with_updated_world(&self, world_key: &WorldKey, world: World) -> Self {
        Universe {
            worlds: self.worlds.update(world_key.clone(), world),
            fleets: self.fleets.clone(),
            gates: self.gates.clone(),
        }
    } 

    pub fn get_world(&self, world_key: &WorldKey) -> Result<World, String> {
        match self.worlds.get(&world_key) {
            Some(world) => Ok(world.clone()),
            None => Err(String::from("World not found"))
        }
    }

    pub fn get_fleet(&self, fleet_key: &FleetKey) -> Result<Fleet, String> {
        match self.fleets.get(&fleet_key) {
            Some(fleets) => Ok(fleets.clone()),
            None => Err(String::from("Fleet not found"))
        }
    }

    pub fn parse_print_out(print_out: &String) -> Result<Self,String> {
        let mut gates: HashSet<Gate> = HashSet::new();

        let world_print_outs: Vec<&str> = print_out.trim().split("\n\n").collect();
        for world_print_out in world_print_outs {
            if !world_print_out.starts_with('W') {
                return Err(format!("World line does not start with 'W' but with {}...", &world_print_out[0..3]))
            }

            let lines: Vec<&str> = world_print_out.split("\n").collect();

            let world_parts: Vec<&str> = lines[0].splitn(4, ' ').collect();
            println!("{:?}", world_parts);
            
            let world_key_value: u8 = match (&world_parts[0][1..]).parse() {
                Ok(value) => value,
                Err(_) => return Err(format!("Could not parse world key: {}", &world_parts[0]))
            };
            let world_key = WorldKey::new(world_key_value);

            let gate_parts: Vec<&str> = world_parts[1].trim_matches(|c| c == '(' || c == ')' ).split(',').collect();
            for gate_part in gate_parts {
                let gate_value: u8 = match (&gate_part).parse() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Could not parse gates: {}", &world_parts[1]))
                };
                gates.insert(Gate::new(world_key, WorldKey::new(gate_value)));
            }

            let owner_key_value: String = String::from(world_parts[2].trim_matches(|c| c == '[' || c == ']' ));
            let owner_key = PlayerToken::new(owner_key_value);

            // TODO world internals

            // TODO fleets

            println!("{:?} {:?} {:?}", world_key_value, gates, owner_key);
        }
        Ok(Universe::new())
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (world_key, world) in &self.worlds {
            write!(f,"W{} (", world_key)?;
            for gate in self.gates.iter()
                    .filter(|gate| gate.has_world(&world_key)) {
                write!(f, "{}", gate.other_key(&world_key))?;
            }
            write!(f, ") {}\n", world)?;
            for (fleet_key,fleet) in self.fleets.iter()
                    .filter(|(_,fleet)| fleet.world == world_key.clone()) {
                write!(f, "F{}{}\n", fleet_key, fleet)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "universe")?;
        Ok(())
    }
}