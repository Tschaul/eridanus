use std::ops;

pub struct Number {
    value: u8,
}

impl Number {
    pub fn new(value: u8) -> Self {
        Number { value: value }
    }
}

impl ops::Add<Number> for Number {
    type Output = Number;
    fn add(self, other: Number) -> Number {
        if self.value > u8::max_value() - other.value {
            Number { value: 255 }
        } else {
            Number {
                value: self.value + other.value,
            }
        }
    }
}

impl ops::Sub<Number> for Number {
    type Output = Number;
    fn sub(self, other: Number) -> Number {
        if self.value < other.value {
            Number { value: 0 }
        } else {
            Number {
                value: self.value - other.value,
            }
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
