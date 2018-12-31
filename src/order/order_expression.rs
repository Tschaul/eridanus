use crate::model::player::PlayerToken;

#[derive(Clone,Debug)]
pub enum OrderChar {
    W,
    F,
    T,
    I,
    A,
    P,
}

#[derive(Clone,Debug)]
pub enum OrderToken {
    Char(OrderChar),
    Num(u8),
    None,
}

pub type OrderTokens = (OrderToken,OrderToken,OrderToken,OrderToken,OrderToken,OrderToken);

#[derive(Debug)]
pub struct OrderExpression {
    pub tokens: OrderTokens,
    pub player: PlayerToken
}

impl OrderExpression {
    pub fn new(order_str: &str, player: &PlayerToken) -> Option<Self> {

        let mut order_char_tokens: Vec<OrderChar> = Vec::new(); 

        for char_str in order_str.split(|c: char|c.is_digit(10)) {
            match char_str {
                "W" => {order_char_tokens.push(OrderChar::W);},
                "F" => {order_char_tokens.push(OrderChar::F);},
                "T" => {order_char_tokens.push(OrderChar::T);},
                "I" => {order_char_tokens.push(OrderChar::I);},
                "A" => {order_char_tokens.push(OrderChar::A);},
                "P" => {order_char_tokens.push(OrderChar::P);},
                _ => { }
            }
        }

        let mut order_num_tokens: Vec<u8> = Vec::new();
        for char_str in order_str.split(|c: char|!c.is_digit(10)) {
            match char_str.parse() {
                Ok(num) => {
                    order_num_tokens.push(num);
                },
                Err(_) => { }
            } 
        }

        let token_1: OrderToken = match order_char_tokens.get(0) {
            Some(c) => OrderToken::Char(c.clone()),
            None => OrderToken::None,
        };

        let token_2: OrderToken = match order_num_tokens.get(0) {
            Some(c) => OrderToken::Num(c.clone()),
            None => OrderToken::None,
        };

        let token_3: OrderToken = match order_char_tokens.get(1) {
            Some(c) => OrderToken::Char(c.clone()),
            None => OrderToken::None,
        };

        let token_4: OrderToken = match order_num_tokens.get(1) {
            Some(c) => OrderToken::Num(c.clone()),
            None => OrderToken::None,
        };

        let token_5: OrderToken = match order_char_tokens.get(2) {
            Some(c) => OrderToken::Char(c.clone()),
            None => OrderToken::None,
        };

        let token_6: OrderToken = match order_num_tokens.get(2) {
            Some(c) => OrderToken::Num(c.clone()),
            None => OrderToken::None,
        };

        Some(OrderExpression { 
            tokens: (
                token_1,
                token_2,
                token_3,
                token_4,
                token_5,
                token_6
            ),
            player: player.clone()
        })
    }

}