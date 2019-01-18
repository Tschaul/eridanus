use crate::model::base_types::Amount;
use crate::model::fleet::FleetKey;
use crate::model::player::PlayerToken;
use crate::model::universe::Universe;
use crate::model::world::WorldKey;
use crate::order::order::Order;
use crate::order::order::OrderType;
use crate::order::order_expression::OrderChar;
use crate::order::order_expression::OrderExpression;
use crate::order::order_expression::OrderToken;

#[derive(Clone, Debug)]
pub struct MoveShipOrder {
    player: PlayerToken,
    fleet_key: FleetKey,
    destination_key: WorldKey,
    second_destination_key: Option<WorldKey>,
    third_destination_key: Option<WorldKey>,
}

impl Order for MoveShipOrder {
    fn get_order_type(&self) -> OrderType {
        OrderType::MoveShipOrder
    }

    fn execute(&self, universe: &Universe) -> Result<Universe, String> {
        let u = self.move_ship(universe, &self.destination_key.clone())?;
        match self.second_destination_key {
            None => Ok(u),
            Some(k2) => {
                let u2 = self.move_ship(&u.clone(), &k2.clone())?;
                match self.third_destination_key {
                    None => Ok(u2),
                    Some(k3) => {
                        let u3 = self.move_ship(&u2.clone(), &k3.clone())?;
                        Ok(u3)
                    }
                }
            }
        }
    }
}

impl MoveShipOrder {
    pub fn try_parse(order: &OrderExpression) -> Option<Box<Order>> {
        match order.tokens {
            (
                OrderToken::Char(OrderChar::F),
                OrderToken::Num(f),
                OrderToken::Char(OrderChar::W),
                OrderToken::Num(w),
                OrderToken::None,
                OrderToken::None,
                OrderToken::None,
                OrderToken::None,
            ) => Some(Box::new(MoveShipOrder {
                player: order.player.clone(),
                fleet_key: FleetKey::new(f.clone()),
                destination_key: WorldKey::new(w.clone()),
                second_destination_key: None,
                third_destination_key: None,
            })),
            (
                OrderToken::Char(OrderChar::F),
                OrderToken::Num(f),
                OrderToken::Char(OrderChar::W),
                OrderToken::Num(w1),
                OrderToken::Char(OrderChar::W),
                OrderToken::Num(w2),
                OrderToken::None,
                OrderToken::None,
            ) => Some(Box::new(MoveShipOrder {
                player: order.player.clone(),
                fleet_key: FleetKey::new(f.clone()),
                destination_key: WorldKey::new(w1.clone()),
                second_destination_key: Some(WorldKey::new(w2.clone())),
                third_destination_key: None,
            })),
            (
                OrderToken::Char(OrderChar::F),
                OrderToken::Num(f),
                OrderToken::Char(OrderChar::W),
                OrderToken::Num(w1),
                OrderToken::Char(OrderChar::W),
                OrderToken::Num(w2),
                OrderToken::Char(OrderChar::W),
                OrderToken::Num(w3),
            ) => Some(Box::new(MoveShipOrder {
                player: order.player.clone(),
                fleet_key: FleetKey::new(f.clone()),
                destination_key: WorldKey::new(w1.clone()),
                second_destination_key: Some(WorldKey::new(w2.clone())),
                third_destination_key: Some(WorldKey::new(w3.clone())),
            })),
            _ => None,
        }
    }

    pub fn move_ship(
        &self,
        universe: &Universe,
        destination_key: &WorldKey,
    ) -> Result<Universe, String> {
        let mut fleet = universe.get_fleet(&self.fleet_key)?;
        println!("XXXXXXXXXXXXXX moving fleet {:?}", fleet);
        if !universe.has_gate(&fleet.world.clone(), &destination_key.clone()) {
            return Err(String::from("No gate to destination"));
        }
        fleet.world = destination_key.clone();
        Ok(universe.with_updated_fleet(&self.fleet_key, fleet))
    }
}
