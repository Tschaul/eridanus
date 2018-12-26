mod model;
mod orders;

use crate::model::base_types::Amount;
use crate::model::world::World;
use crate::model::world::WorldKey;
use crate::model::player::PlayerToken;
use crate::model::universe::Universe;

fn main() {
    let print_out = String::from("
W75 (5,12,86) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
F3[TERRAN]=0
F70[TERRAN]=0
F102[TERRAN]=0
F119[TERRAN]=0
F133[TERRAN]=0

W5 (75,12) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
");

    let universe_result = Universe::parse_print_out(&print_out);

    println!("done parsing");
    match universe_result {
        Ok(universe) => {
            println!("{}", universe);
        },
        Err(err) => println!("{}",err)
    }
}
