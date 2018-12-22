#[cfg(test)]
mod tests {

    use crate::model::base_types::Amount;

    #[test]
    fn test_add() {
        assert_eq!(Amount::new(1) + Amount::new(2), Amount::new(3));
        assert_eq!(Amount::new(245) + Amount::new(10), Amount::new(255));
    }

    #[test]
    fn test_add_overflow() {
        assert_eq!(Amount::new(245) + Amount::new(11), Amount::new(255));
    }

    #[test]
    fn test_sub() {
        assert_eq!(Amount::new(3) - Amount::new(2), Amount::new(1));
    }

    #[test]
    fn test_sub_underflow() {
        assert_eq!(Amount::new(2) - Amount::new(3), Amount::new(0));
    }
}