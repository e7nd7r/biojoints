use std::convert::From;

pub struct Order {
    pub class: String,
    pub order: String,
    pub subclass: String,
    pub superorder: String,
}

pub type OrderRecord = (String, String, String, String);

impl From<OrderRecord> for Order {
    fn from(value: OrderRecord) -> Self {
        let (class, order, subclass, superorder) = value;

        Self {
            class,
            order,
            subclass,
            superorder,
        }
    }
}