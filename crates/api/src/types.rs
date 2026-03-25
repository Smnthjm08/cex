use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Copy, Clone, PartialEq)]
pub enum OrderSide {
    BUY,
    SELL,
}
