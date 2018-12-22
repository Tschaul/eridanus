use crate::orders::order::OrderType;
use crate::orders::order::Order;
use crate::model::fleet::FleetKey;
use crate::model::world::WorldKey;
use crate::model::base_types::Amount;
use crate::model::universe::Universe;

enum Source {
    FromShip(FleetKey)
}

enum Target {
    ToShip(FleetKey)
}

pub struct TransferOrder {
    source: Source,
    target: Target,
    amount: Amount,
}

impl Order for TransferOrder {
    fn order_type(&self) -> OrderType {
        OrderType::TransferOrder
    }

    fn execute(&self, mut universe: Universe) -> Result<(),String> {
        let world_key = std::try!(self.draw_from_source(&mut universe));
        std::try!(self.push_to_target(&mut universe, world_key));
        Ok(())
    }
}

impl TransferOrder {
    fn draw_from_source(&self, universe: &mut Universe) -> Result<WorldKey,String> {
        match &self.source {
            Source::FromShip(fleet_key) => {
                match universe.fleets.get_mut(fleet_key) {
                    Some(fleet) => {
                        let world_from = fleet.world;
                        if self.amount > fleet.ships {
                            return Err("Not enough ships".to_string()) 
                        } else {
                            fleet.ships = fleet.ships - self.amount;
                            Ok(world_from)
                        }
                    },
                    None => {
                        return Err("Fleet not found".to_string())
                    }
                }

            }
        }
    }

    fn push_to_target(&self, universe: &mut Universe, world_key: WorldKey) -> Result<(),String> {
        match &self.target {
            Target::ToShip(fleet_key) => {
                match universe.fleets.get_mut(fleet_key) {
                    Some(fleet) => {
                        let world_to = fleet.world;
                        if world_to != world_key {
                            Err("Source and target in different worlds".to_string())
                        } else {
                            fleet.ships = fleet.ships + self.amount;
                            Ok(())
                        }
                    },
                    None => Err("Fleet not found".to_string())
                }
            }
        }
    }
}