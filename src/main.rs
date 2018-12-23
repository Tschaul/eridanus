mod model;
mod orders;

use crate::model::base_types::Amount;
use crate::model::world::World;
use crate::model::world::WorldKey;
use crate::model::player::PlayerToken;
use crate::model::universe::Universe;

fn main() {
    println!("Hello, world!");

    let n1 = Amount::new(245);

    let n2 = Amount::new(10);

    println!("2 - 3 = {}",n2-n1);

    let world = World {
        owner: Some(PlayerToken::new("TERRAN".to_string())),
        industry: Amount::new(10),
        population: Amount::new(10),
        i_ships: Amount::new(10),
        p_ships: Amount::new(10),
        turns: Amount::new(1),
        limit: Amount::new(1),
        metal: Amount::new(1),
        mines: Amount::new(1),
    };

    println!("{}", world);

    let universe = Universe::new().with_updated_world(&WorldKey::new(1), world);

    println!("asd");
    println!("{}", universe);

    let print_out = String::from("
W75 (5,12,86) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
F3[TERRAN]=0
F70[TERRAN]=0
F102[TERRAN]=0
F119[TERRAN]=0
F133[TERRAN]=0

W5 (75,12) [TERRAN] (Industry=30, Metal=30, Mines=2, Population=50, Limit=100, Turns=1, I-Ships=1, P-Ships=1)
");

    Universe::parse_print_out(&print_out);
}
