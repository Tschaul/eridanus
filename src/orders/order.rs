use crate::model::universe::Universe;

pub enum OrderType {
    TransferOrder
}

pub trait Order: Sized {
    fn get_order_type(&self) -> OrderType;

    fn execute(&self, universe: Universe) -> Result<Universe, String>;

    fn try_parse(order: &String) -> Option<Self>;
}