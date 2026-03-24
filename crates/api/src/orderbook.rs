use crate::types::OrderSide;
use std::collections::HashMap;

pub struct OrderBook {
    pub bids: HashMap<u32, Vec<(i32, i32)>>,
    pub asks: HashMap<u32, Vec<(i32, i32)>>,
    pub order_id_index: u32,
}

pub struct UserOrder {
    user_id: u32,
    qty: u32,
    order_id: u32,
}

impl OrderBook {
    pub fn new() -> Self {
        Self {
            bids: HashMap::new(),
            asks: HashMap::new(),
            order_id_index: 0,
        }
    }
}

// TODO pending
impl OrderBook {
    pub fn create_order(&mut self, price: u32, qty: u32, user_id: u32, side: OrderSide) {
        // if side == OrderSide::BUY{
        //     self.bids.iter().f
        // } else{

        // }
    }
}
