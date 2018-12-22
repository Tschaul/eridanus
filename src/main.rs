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

    let mut universe = Universe::new();

    universe.worlds.insert(WorldKey::new(1), world);
}
