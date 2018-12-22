use std::collections::HashMap;
use std::collections::HashSet;
use crate::model::fleet::Fleet;
use crate::model::fleet::FleetKey;
use crate::model::world::World;
use crate::model::world::WorldKey;
use crate::model::gate::Gate;

pub struct Universe {
    pub fleets: HashMap<FleetKey, Fleet>,
    pub worlds: HashMap<WorldKey, World>,
    pub gates: HashSet<Gate>
}

impl Universe {
    pub fn new() -> Self {
        Universe {
            fleets: HashMap::new(),
            worlds: HashMap::new(),
            gates: HashSet::new()
        }
    }
}