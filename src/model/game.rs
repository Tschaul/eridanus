use crate::model::universe::Universe;

#[derive(Debug, PartialEq, Eq)]
pub struct Game {
    universe: Universe
}

impl Game {

    pub fn get_universe(&self) -> Universe {
        self.universe.clone()
    }

    pub fn with_updated_universe(&self, universe: Universe) -> Self {
        Game {
            universe: universe
        }
    }

    pub fn parse_print_out(print_out: &str) -> Result<Self, String> {
        let pos = match print_out.find("#UNIVERSE") {
            Some(number) => number,
            None => return Err(format!("#UNIVERSE section not found in game print out"))
        };

        let universe = Universe::parse_print_out(&print_out[(pos+9)..])?;

        Ok(Game {
            universe: universe
        })
    }
}