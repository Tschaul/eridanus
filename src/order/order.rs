use crate::model::universe::Universe;

pub enum OrderType {
    TransferOrder
}

pub trait Order : std::fmt::Debug {
    fn get_order_type(&self) -> OrderType;

    fn execute(&self, universe: &Universe) -> Result<Universe, String>;
}