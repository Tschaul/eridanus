#[cfg(test)]
mod transfer_ships_tests {

    use crate::model::base_types::Amount;
    use crate::model::game::Game;
    use crate::turn::turn::Turn;

    #[test]
    fn ship_to_ship() -> Result<(),String> {

        let game = Game::parse_print_out(&String::from("
#UNIVERSE
W75 () [TERRAN] ()
F3[TERRAN]=10
F70[TERRAN]=0"))?;

        let turn = Turn::empty(1,1).parse_orders(&String::from("
#TURN
[TERRAN] (Turn=1, Game-Nonce=1)
F3T5F70
"))?;

        let game2 = turn.execute_orders(&game);

        assert_eq!(game2, Game::parse_print_out(&String::from("
#UNIVERSE
W75 () [TERRAN] ()
F3[TERRAN]=5
F70[TERRAN]=5"))?);

        Ok(())
    }

}