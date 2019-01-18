#[cfg(test)]
mod move_ship_tests {

    use crate::model::base_types::Amount;
    use crate::model::game::Game;
    use crate::turn::turn::Turn;

    #[test]
    fn happy_case() -> Result<(),String> {

        let game = Game::parse_print_out(&String::from("
#UNIVERSE
W75 (5) [TERRAN] ()
F3[TERRAN]=10

W5 (75) [TERRAN] ()"))?;

        let mut turn = Turn::empty(1,1);

        turn.parse_orders(&String::from("
#TURN
[TERRAN] (Turn=1, Game-Nonce=1)
F3W5
"))?;

        let game2 = turn.execute_orders(&game);

        assert_eq!(game2, Game::parse_print_out(&String::from("
#UNIVERSE
W75 (5) [TERRAN] ()

W5 (75) [TERRAN] ()
F3[TERRAN]=10"))?);

        Ok(())
    }

    
    #[test]
    fn not_connected() -> Result<(),String> {

        let game = Game::parse_print_out(&String::from("
#UNIVERSE
W75 () [TERRAN] ()
F3[TERRAN]=10

W5 () [TERRAN] ()"))?;

        let mut turn = Turn::empty(1,1);

        turn.parse_orders(&String::from("
#TURN
[TERRAN] (Turn=1, Game-Nonce=1)
F3W5
"))?;

        let game2 = turn.execute_orders(&game);

        assert_eq!(game2, Game::parse_print_out(&String::from("
#UNIVERSE
W75 () [TERRAN] ()
F3[TERRAN]=10

W5 () [TERRAN] ()"))?);

        Ok(())
    }

    
    #[test]
    fn move_two_worlds() -> Result<(),String> {

        let game = Game::parse_print_out(&String::from("
#UNIVERSE
W75 (5) [TERRAN] ()
F3[TERRAN]=10

W5 (75,7) [TERRAN] ()

W7 (5) [TERRAN] ()"))?;

        let mut turn = Turn::empty(1,1);

        turn.parse_orders(&String::from("
#TURN
[TERRAN] (Turn=1, Game-Nonce=1)
F3W5W7
"))?;

        let game2 = turn.execute_orders(&game);

        assert_eq!(game2, Game::parse_print_out(&String::from("
#UNIVERSE
W75 (5) [TERRAN] ()

W5 (75,7) [TERRAN] ()

W7 (5) [TERRAN] ()
F3[TERRAN]=10"))?);

        Ok(())
    }

    
    #[test]
    fn move_three_worlds() -> Result<(),String> {

        let game = Game::parse_print_out(&String::from("
#UNIVERSE
W75 (5) [TERRAN] ()
F3[TERRAN]=10

W5 (75,7) [TERRAN] ()

W7 (5,21) [TERRAN] ()

W21 (21) [TERRAN] ()"))?;

        let mut turn = Turn::empty(1,1);

        turn.parse_orders(&String::from("
#TURN
[TERRAN] (Turn=1, Game-Nonce=1)
F3W5W7W21
"))?;

        let game2 = turn.execute_orders(&game);

        assert_eq!(game2, Game::parse_print_out(&String::from("
#UNIVERSE
W75 (5) [TERRAN] ()

W5 (75,7) [TERRAN] ()

W7 (5,21) [TERRAN] ()

W21 (21) [TERRAN] ()
F3[TERRAN]=10"))?);

        Ok(())
    }

}