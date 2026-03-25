use serde::Serialize;

use crate::types::OrderSide;
use std::collections::HashMap;

#[derive(Serialize, Clone)]
pub struct OrderBook {
    pub bids: HashMap<u32, Vec<UserOrder>>,
    pub asks: HashMap<u32, Vec<UserOrder>>,
    pub order_id_index: u32,
}

#[derive(Serialize, Clone)]
pub struct UserOrder {
  pub  user_id: u32,
  pub  quantity: u32,
  pub  order_id: u32,
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

// impl OrderBook {
//     pub fn create_order(&mut self, price: u32, qty: u32, user_id: u32, side: OrderSide) {

//         if side == OrderSide::BUY {
//             let mut bids = self.bids
//                 .entry(price)
//                 .or_insert(Vec::new())
//                 .push(UserOrder {
//                     user_id,
//                     qty,
//                     order_id: 12123,
//                 });
//             bids.order_id
//         } else {
//             self.asks
//                 .entry(price)
//                 .or_insert(Vec::new())
//                 .push(UserOrder {
//                     user_id,
//                     qty,
//                     order_id: 123,
//                 });
//         }
//     }
// }

impl OrderBook {
    pub fn create_order(&mut self, price: u32, quantity: u32, user_id: u32, side: OrderSide) -> u32 {
        self.order_id_index += 1;
        let order_id = self.order_id_index;

        let order = UserOrder {
            user_id,
            quantity,
            order_id,
        };

        match side {
            OrderSide::BUY => {
                self.bids.entry(price).or_default().push(order);
            }
            OrderSide::SELL => {
                self.asks.entry(price).or_default().push(order);
            }
        }

        order_id
    }

    pub fn get_depth(&self) -> OrderBook{
        return OrderBook{
            bids: self.bids.clone(),
            asks: self.asks.clone(),
            order_id_index: self.order_id_index
        }
    }

    
}
