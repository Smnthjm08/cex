use actix_web::{
    App, HttpResponse, HttpServer, Responder, delete, get, post,
    web::{self, Json},
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
    filled_qty: u32,
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
async fn get_depth(data: web::Data<OrderBook>) -> impl Responder {
    HttpResponse::Ok().json(DepthResponse {
        bids: vec![],
        asks: vec![],
        order_id_index: String::from("1232323"),
    })
}

// POST - orders/

#[post("/")]
async fn create_order(
    Json(body): Json<CreateOrderInput>,
    data: web::Data<OrderBook>,
) -> impl Responder {
    let price = body.price;
    let quantity = body.quantity;
    let user_id = body.user_id;
    let side = body.side;

    println!("create order body: {:?}", body);

    // data.into_inner().

    return HttpResponse::Ok().json(CreateOrderResponse {
        order_id: 1,
        price,
        quantity,
        user_id,
        side,
    });
}

// DELETE - orders/

#[delete("/")]
async fn delete_order(
    Json(body): Json<DeleteOrderInput>,
    data: web::Data<OrderBook>,
) -> impl Responder {
    let order_id = body.order_id;

    println!("delete order body: {:?}", order_id);

    return HttpResponse::Ok().json(DeleteOrderResposne {
        filled_qty: 21,
        avg_price: 100,
    });
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(OrderBook::new()))
            .service(
                web::scope("/order")
                    .service(get_depth)
                    .service(create_order)
                    .service(delete_order),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
