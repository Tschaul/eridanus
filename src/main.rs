mod model;
mod order;
mod turn;
mod tests;

use crate::model::base_types::Amount;
use crate::model::world::World;
use crate::model::world::WorldKey;
use crate::model::player::PlayerToken;
use crate::model::game::Game;
use crate::model::universe::Universe;

fn main() {
    let print_out = String::from("
#UNIVERSE
W75 (5,12,86) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
F3[TERRAN]=0
F70[TERRAN]=0
F102[TERRAN]=0
F119[TERRAN]=0
F133[TERRAN]=0

W5 (75,12) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
");

let print_out2 = String::from("
W75 (5,12,86) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
F3[TERRAN]=0
F70[TERRAN]=0
F102[TERRAN]=0
F119[TERRAN]=0
F133[TERRAN]=0

W5 (75,12) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
");

    let game_result = Game::parse_print_out(&print_out);
    let universe_result2 = Universe::parse_print_out(&print_out2);

    println!("done parsing");
    match game_result {
        Ok(game) => {
            match universe_result2 {
                Ok(universe2) => {
                    assert_eq!(game.get_universe(),universe2)
                },
                Err(err) => {println!("{}",err)}
            }
        },
        Err(err) => println!("{}",err)
    }
}
