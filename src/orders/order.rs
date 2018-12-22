use crate::model::universe::Universe;

pub enum OrderType {
    TransferOrder
}

pub trait Order {
    fn order_type(&self) -> OrderType;

    fn execute(&self, universe: Universe) -> Result<(), String>;
}