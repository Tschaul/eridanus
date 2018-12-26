extern crate im_rc;

use im_rc::HashMap;
use im_rc::HashSet;

use crate::model::fleet::Fleet;
use crate::model::fleet::FleetKey;
use crate::model::world::World;
use crate::model::world::WorldKey;
use crate::model::gate::Gate;

pub struct Universe {
    fleets: HashMap<FleetKey, Fleet>,
    worlds: HashMap<WorldKey, World>,
    gates: HashSet<Gate>
}

impl Universe {

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
        let mut worlds: HashMap<WorldKey, World> = HashMap::new();
        let mut fleets: HashMap<FleetKey, Fleet> = HashMap::new();

        let world_print_outs: Vec<&str> = print_out.trim().split("\n\n").collect();
        for world_print_out in world_print_outs {
            if !world_print_out.starts_with('W') {
                return Err(format!("World line does not start with 'W' but with {}...", &world_print_out[0..3]))
            }

            let lines: Vec<&str> = world_print_out.split("\n").collect();

            let world_parts: Vec<&str> = lines[0].splitn(3, ' ').collect();
            
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

            println!("{:?}", gates);

            let world = World::parse_print_out(world_parts[2])?;

            worlds.insert(world_key, world);

            for line in &lines[1..]  {
                let pos = match line.find('[') {
                    Some(number) => number,
                    None => return Err(format!("Bad format for fleet 1: {}", line))
                };
                println!("{} , {} , {}", line, pos, &line[1..pos]);
                let flee_key_value: u8 = match line[1..(pos)].parse() {
                    Ok(value) => value,
                    Err(_) => return Err(format!("Bad format for fleet 2: {}", &line[1..pos]))
                };
                let fleet_key = FleetKey::new(flee_key_value);
                let fleet = Fleet::parse_print_out(&line[pos..], world_key)?;

                fleets.insert(fleet_key, fleet);
            }

        }

        Ok(Universe {
            worlds: worlds,
            fleets: fleets,
            gates: gates,
        })
    }
}

impl std::fmt::Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (world_key, world) in &self.worlds {
            write!(f,"W{} (", world_key)?;
            let gates: Vec<String> = self.gates.iter()
                .filter(|gate| gate.has_world(&world_key))
                .map(|gate| gate.other_key(&world_key))
                .map(|world_key| format!("{}", world_key))
                .collect();
            write!(f, "{}", gates.join(","))?;
            write!(f, ") {}\n", world)?;
            for (fleet_key,fleet) in self.fleets.iter()
                    .filter(|(_,fleet)| fleet.world == world_key.clone()) {
                write!(f, "F{}{}\n", fleet_key, fleet)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}