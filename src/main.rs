use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Clone)]
struct Item {
    id: u32,
    name: String,
}

type Items = web::Data<Mutex<HashMap<u32, Item>>>;

#[get("/items")]
async fn get_items(items: Items) -> impl Responder {
    let data = items.lock().unwrap();
    let items_vec: Vec<Item> = data.values().cloned().collect();
    HttpResponse::Ok().json(items_vec)
}

#[post("/items")]
async fn create_item(item: web::Json<Item>, items: Items) -> impl Responder {
    let mut data = items.lock().unwrap();
    data.insert(item.id, item.into_inner());
    HttpResponse::Created().body("Item created")
}

#[put("/items/{id}")]
async fn update_item(id: web::Path<u32>, item: web::Json<Item>, items: Items) -> impl Responder {
    let mut data = items.lock().unwrap();
    if let Some(existing_item) = data.get_mut(&id) {
        *existing_item = item.into_inner();
        HttpResponse::Ok().body("Item updated")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

#[delete("/items/{id}")]
async fn delete_item(id: web::Path<u32>, items: Items) -> impl Responder {
    let mut data = items.lock().unwrap();
    if data.remove(&id).is_some() {
        HttpResponse::Ok().body("Item deleted")
    } else {
        HttpResponse::NotFound().body("Item not found")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let items: Items = web::Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        App::new()
            .app_data(items.clone())
            .service(get_items)
            .service(create_item)
            .service(update_item)
            .service(delete_item)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}