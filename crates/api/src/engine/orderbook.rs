// engine/OrderBook
use crate::types::order_types::{Order, OrderSide};
use std::collections::BTreeMap;

pub struct OrderBook {
    pub bids: BTreeMap<f64, Vec<Order>>,
    pub asks: BTreeMap<f64, Vec<Order>>,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
        }
    }
}

impl OrderBook{
    pub fn create_order(&mut self, order: Order){
        match order.side {
            OrderSide::Buy => {
                // self.bids.entry(order.price).or_default().push(order)
            },
            OrderSide::Sell => {
                // self.asks.entry(order.price).or_default().push(order)
            }
        }
    }
}
