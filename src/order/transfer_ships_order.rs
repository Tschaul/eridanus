use crate::order::order::OrderType;
use crate::order::order::Order;
use crate::model::fleet::FleetKey;
use crate::model::world::WorldKey;
use crate::model::player::PlayerToken;
use crate::model::base_types::Amount;
use crate::model::universe::Universe;
use crate::order::order_expression::OrderExpression;
use crate::order::order_expression::OrderToken;
use crate::order::order_expression::OrderChar;

#[derive(Clone,Debug)]
enum Source {
    FromFleet(FleetKey),
    FromIShip(WorldKey),
    FromPShip(WorldKey)
}

#[derive(Clone,Debug)]
enum Target {
    ToFleet(FleetKey),
    ToIShip,
    ToPShip
}

#[derive(Clone, Debug)]
pub struct TransferShipsOrder {
    player: PlayerToken,
    source: Source,
    target: Target,
    amount: Amount,
}

impl Order for TransferShipsOrder {
    fn get_order_type(&self) -> OrderType {
        OrderType::TransferOrder
    }

    fn execute(&self, universe: &Universe) -> Result<Universe,String> {
        let (world_key, u1) = self.draw_from_source(universe)?;
        let u2 = self.push_to_target(&u1, world_key)?;
        Ok(u2)
    }
}

impl TransferShipsOrder {

    pub fn try_parse(order: &OrderExpression) -> Option<Box<Order>> {
        match order.tokens {
            (
                OrderToken::Char(OrderChar::F),
                OrderToken::Num(f1),
                OrderToken::Char(OrderChar::T),
                OrderToken::Num(a),
                OrderToken::Char(OrderChar::F),
                OrderToken::Num(f2),
                OrderToken::None,
                OrderToken::None
            ) => Some(Box::new(TransferShipsOrder {
                player: order.player.clone(),
                source: Source::FromFleet(FleetKey::new(f1.clone())),
                target: Target::ToFleet(FleetKey::new(f2.clone())),
                amount: Amount::new(a.clone())
            })),
            (
                OrderToken::Char(OrderChar::F),
                OrderToken::Num(f1),
                OrderToken::Char(OrderChar::T),
                OrderToken::Num(a),
                OrderToken::Char(OrderChar::I),
                OrderToken::None,
                OrderToken::None,
                OrderToken::None
            ) => Some(Box::new(TransferShipsOrder {
                player: order.player.clone(),
                source: Source::FromFleet(FleetKey::new(f1.clone())),
                target: Target::ToIShip,
                amount: Amount::new(a.clone())
            })),
            (
                OrderToken::Char(OrderChar::F),
                OrderToken::Num(f1),
                OrderToken::Char(OrderChar::T),
                OrderToken::Num(a),
                OrderToken::Char(OrderChar::P),
                OrderToken::None,
                OrderToken::None,
                OrderToken::None
            ) => Some(Box::new(TransferShipsOrder {
                player: order.player.clone(),
                source: Source::FromFleet(FleetKey::new(f1.clone())),
                target: Target::ToPShip,
                amount: Amount::new(a.clone())
            })),
            _ => None
        }
    }

    fn draw_from_source(&self, universe: &Universe) -> Result<(WorldKey,Universe),String> {
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

    fn push_to_target(&self, universe: &Universe, world_key: WorldKey) -> Result<Universe,String> {
        match &self.target {
            Target::ToFleet(fleet_key) => {
                let mut fleet = universe.get_fleet(fleet_key)?; 
                let world_to = fleet.world;
                if world_to != world_key {
                    Err(String::from("Source and target are in different worlds"))
                } else {
                    fleet.ships = fleet.ships + self.amount;
                    Ok(universe.with_updated_fleet(fleet_key, fleet))
                }
            },
            Target::ToIShip => {
                let mut world = universe.get_world(&world_key)?;
                world.i_ships = world.i_ships + self.amount;
                Ok(universe.with_updated_world(&world_key, world))
            },
            Target::ToPShip => {
                let mut world = universe.get_world(&world_key)?;
                world.p_ships = world.p_ships + self.amount;
                Ok(universe.with_updated_world(&world_key, world))                
            }
        }
    }
}