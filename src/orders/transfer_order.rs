use crate::orders::order::OrderType;
use crate::orders::order::Order;
use crate::model::fleet::FleetKey;
use crate::model::world::WorldKey;
use crate::model::player::PlayerToken;
use crate::model::base_types::Amount;
use crate::model::universe::Universe;

enum Source {
    FromFleet(FleetKey),
    FromIShip(WorldKey),
    FromPShip(WorldKey)
}

enum Target {
    ToFleet(FleetKey),
    ToIShip,
    ToPShip
}

pub struct TransferOrder {
    player: PlayerToken,
    source: Source,
    target: Target,
    amount: Amount,
}

impl Order for TransferOrder {
    fn get_order_type(&self) -> OrderType {
        OrderType::TransferOrder
    }

    fn execute(&self, universe: Universe) -> Result<Universe,String> {
        let (world_key, u1) = self.draw_from_source(universe)?;
        // let u2 = self.push_to_target(&u1, world_key)?;
        Ok(u1)
    }
}

impl TransferOrder {
    fn draw_from_source(&self, universe: Universe) -> Result<(WorldKey,Universe),String> {
        match &self.source {
            Source::FromFleet(fleet_key) => {
                let mut fleet = universe.get_fleet(fleet_key)?;
                if fleet.owner != Some(self.player.clone()) {
                    return Err(String::from("Source fleet not owned by player"))
                }
                let world_from = fleet.world;
                if self.amount > fleet.ships {
                    Err(String::from("Not enough ships"))
                } else {
                    fleet.ships = fleet.ships - self.amount;
                    Ok((
                        world_from,
                        universe.with_updated_fleet(fleet_key, fleet)
                    ))
                }
            },
            Source::FromIShip(world_key) => {
                let mut world = universe.get_world(world_key)?;
                if self.amount > world.i_ships {
                    Err(String::from("Not enough ships"))
                } else {
                    world.i_ships = world.i_ships - self.amount;
                    Ok((
                        world_key.clone(),
                        universe.with_updated_world(world_key, world)
                    ))
                }
            },
            Source::FromPShip(world_key) => {
                let mut world = universe.get_world(world_key)?;
                if self.amount > world.p_ships {
                    Err(String::from("Not enough ships"))
                } else {
                    world.p_ships = world.p_ships - self.amount;
                    Ok((
                        world_key.clone(),
                        universe.with_updated_world(world_key, world)
                    ))
                }
            }
        }
    }

    // fn push_to_target(mut self, universe: &Universe, world_key: WorldKey) -> Result<(),String> {
    //     match &self.target {
    //         Target::ToFleet(fleet_key) => {
    //             match universe.fleets.get_mut(fleet_key) {
    //                 Some(fleet) => {
    //                     let world_to = fleet.world;
    //                     if world_to != world_key {
    //                         Err(String::from("Source and target are in different worlds"))
    //                     } else {
    //                         fleet.ships = fleet.ships + self.amount;
    //                         Ok(())
    //                     }
    //                 },
    //                 None => Err(String::from("Fleet not found"))
    //             }
    //         },
    //         Target::ToIShip => {
    //             match universe.worlds.get_mut(&world_key) {
    //                 Some(world) => {
    //                     world.i_ships = world.i_ships + self.amount;
    //                     Ok(())
    //                 },
    //                 None => Err(String::from("World not found"))
    //             }
    //         },
    //         Target::ToPShip => {
    //             match universe.worlds.get_mut(&world_key) {
    //                 Some(world) => {
    //                     world.p_ships = world.p_ships + self.amount;
    //                     Ok(())
    //                 },
    //                 None => Err(String::from("World not found"))
    //             }
    //         }
    //     }
    // }
}