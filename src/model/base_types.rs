use std::ops;

pub struct Amount {
    value: u8,
}

impl Amount {
    pub fn new(value: u8) -> Self {
        Amount { value: value }
    }
}

impl ops::Add<Amount> for Amount {
    type Output = Amount;
    fn add(self, other: Amount) -> Amount {
        if self.value > u8::max_value() - other.value {
            Amount { value: 255 }
        } else {
            Amount {
                value: self.value + other.value,
            }
        }
    }
}

impl ops::Sub<Amount> for Amount {
    type Output = Amount;
    fn sub(self, other: Amount) -> Amount {
        if self.value < other.value {
            Amount { value: 0 }
        } else {
            Amount {
                value: self.value - other.value,
            }
        }
    }
}

impl ToString for Amount {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
