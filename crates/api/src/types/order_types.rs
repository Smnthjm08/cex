use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    Limit,
    Market,
}

// {
//     "symbol": "SOLUSDT",
//     "price": 13,
//     "quantity": 1,
//     "side": "SELL",
//     "order_type": "LIMIT"
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    pub symbol: String,
    pub price: f64,
    pub quantity: f64,
    pub side: OrderSide,
    pub order_type: OrderType,
}
