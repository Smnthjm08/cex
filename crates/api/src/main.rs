use std::sync::Mutex;

use actix_web::{
    delete, get, post,
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{orderbook::OrderBook, types::OrderSide};
mod orderbook;
mod types;

#[derive(Debug, Serialize)]
struct CreateOrderResponse {
    price: u32,
    quantity: u32,
    user_id: u32,
    side: OrderSide,
    order_id: u32,
}

#[derive(Debug, Deserialize)]
struct DeleteOrderInput {
    order_id: u32,
}

#[derive(Debug, Serialize)]
struct DeleteOrderResposne {
    filled_quantity: u32,
    avg_price: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DepthResponse {
    bids: Vec<[u32; 2]>,
    asks: Vec<[u32; 2]>,
    order_id_index: String,
}

#[derive(Debug, Deserialize)]
struct CreateOrderInput {
    price: u32,
    quantity: u32,
    user_id: u32,
    side: OrderSide,
}

// GET - orders/depth

#[get("/depth")]
async fn get_depth(data: web::Data<Mutex<OrderBook>>) -> impl Responder {
    let ob = data.lock().unwrap();
    let depth = ob.get_depth();
    HttpResponse::Ok().json(depth)
}

// POST - orders/

#[post("/")]
async fn create_order(
    Json(body): Json<CreateOrderInput>,
    data: web::Data<Mutex<OrderBook>>,
) -> impl Responder {
    let mut ob = data.lock().unwrap();
    // let price = body.price;
    // let quantity = body.quantity;
    // let user_id = body.user_id;
    // let side = body.side;

    println!("create order body: {:?}", body);

    let order_id = ob.create_order(body.price, body.quantity, body.user_id, body.side);

    HttpResponse::Ok().json(CreateOrderResponse {
        order_id,
        price: body.price,
        quantity: body.quantity,
        user_id: body.user_id,
        side: body.side,
    })
}

// DELETE - orders/

#[delete("/")]
async fn delete_order(
    Json(body): Json<DeleteOrderInput>,
    _data: web::Data<Mutex<OrderBook>>,
) -> impl Responder {
    let order_id = body.order_id;

    println!("delete order body: {:?}", order_id);

    return HttpResponse::Ok().json(DeleteOrderResposne {
        filled_quantity: 21,
        avg_price: 100,
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let orderbook = web::Data::new(Mutex::new(OrderBook::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(orderbook.clone())
            .service(
                web::scope("api/v0/order")
                    .service(get_depth)
                    .service(create_order)
                    .service(delete_order),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
