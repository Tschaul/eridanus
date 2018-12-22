use std::ops;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Amount(u8);

impl Amount {
    pub fn new(value: u8) -> Self {
        Amount(value)
    }
}

impl std::fmt::Display for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let Amount(self_value) = self;
        write!(f, "{}", self_value)
    }
}

impl ops::Add<Amount> for Amount {
    type Output = Amount;
    fn add(self, other: Amount) -> Amount {
        let Amount(self_value) = self;
        let Amount(other_value) = other;
        if self_value > u8::max_value() - other_value {
            Amount(255)
        } else {
            Amount(self_value + other_value)
        }
    }
}

impl ops::Sub<Amount> for Amount {
    type Output = Amount;
    fn sub(self, other: Amount) -> Amount {
        let Amount(self_value) = self;
        let Amount(other_value) = other;
        if self_value < other_value {
            Amount(0)
        } else {
            Amount(self_value - other_value)
        }
    }
}
