use std::collections::HashSet;

use crate::model::player::PlayerToken;
use crate::model::universe::Universe;
use crate::model::game::Game;
use crate::order::order::Order;
use crate::order::transfer_ships_order::TransferShipsOrder;
use crate::order::move_ship_order::MoveShipOrder;
use crate::order::order_expression::OrderExpression;

pub struct Turn {
    turn_number: u32,
    game_nonce: u64,
    orders: Vec<OrderExpression>,
    players: HashSet<PlayerToken>
}

impl Turn {

    pub fn empty(turn_number: u32, game_nonce: u64) -> Self {
        Turn {
            turn_number: turn_number,
            game_nonce: game_nonce,
            orders: Vec::new(),
            players: HashSet::new()
        }
    }

    pub fn parse_orders(&mut self, print_out: &str) -> Result<(), String> {
        let lines: Vec<&str> = print_out.trim().split("\n").collect();
        if !lines[0].starts_with("#TURN") {
            return Err(String::from("turn sheet must start with #TURN"))
        }
        let player_split: Vec<&str> = lines[1].splitn(2, ' ').collect();

        let player_key_value: String = String::from(player_split[0].trim_matches(|c| c == '[' || c == ']' ));

        if player_key_value.is_empty() {
            return Err(String::from("no player token found"))
        }

        let player_key = PlayerToken::new(player_key_value);

        if self.players.contains(&player_key) {
           return Err(String::from("orders of player were allready given")) 
        } else {
            self.players.insert(player_key.clone());
        }

        let turn_parts: Vec<&str> = player_split[1].trim_matches(|c| c == '(' || c == ')' ).split(',').collect();
        
        let mut game_nonce: u64 = 0;
        let mut turn_number: u32 = 0;

        for turn_part in turn_parts {
            let split: Vec<&str> = turn_part.split('=').collect();
            match split[0].trim() as &str {
                "Turn" => {
                    let value: u32 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse turn number: {}", &split[1]))
                    };
                    turn_number = value
                },
                "Game-Nonce" => {
                    let value: u64 = match (&split[1].trim()).parse() {
                        Ok(value) => value,
                        Err(_) => return Err(format!("Could not parse game nonce: {}", &split[1]))
                    };
                    game_nonce = value
                },
                _ => {}
            }
        }

        if turn_number != self.turn_number {
            return Err(String::from("turn number does not match"))
        }

        if game_nonce != self.game_nonce {
            return Err(String::from("game nonce does not match"))
        }

        let mut order_list: Vec<Box<Order>> = Vec::new();

        for order_line in &lines[1..] {
            let order_expression = OrderExpression::new(&order_line, &player_key);
            match order_expression {
                Some(expr) => {
                    self.orders.push(expr);
                },
                None => {}
            }
        }

        Ok(())
    }

    fn all_orders_for_type(&self, f: &Fn(&OrderExpression) -> Option<Box<Order>>) -> Vec<Box<Order>> {
        let mut orders: Vec<Box<Order>> = Vec::new();

        for order in &self.orders {
            match f(order) {
                Some(order) => orders.push(order),
                _ => { }
            }
        }

        orders
    }

    pub fn execute_orders(&self, game: &Game) -> Game {
        let mut universe: Universe = game.get_universe();

        let mut orders = self.all_orders_for_type(&TransferShipsOrder::try_parse);
        orders.append(&mut self.all_orders_for_type(&MoveShipOrder::try_parse));

        for order in orders {
            let maybe_universe = order.execute(&universe);
            match maybe_universe {
                Ok(new_universe) => {
                    universe = new_universe;
                },
                Err(error) => {
                    println!("{}", error);
                }
            }
        }

        game.with_updated_universe(universe)
    }
}