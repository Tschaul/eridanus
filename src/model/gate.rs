use crate::model::world::WorldKey;

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
pub struct Gate(WorldKey, WorldKey);

impl Gate {
    pub fn new(world1: WorldKey, world2: WorldKey) -> Self {
        if world1 < world2 {
            Gate(world1, world2)
        } else {
            Gate(world2, world1)
        }
    }
    pub fn has_world(&self, world: &WorldKey) -> bool {
        let Gate(world1, world2) = self;
        world1 == world || world2 == world
    }
}