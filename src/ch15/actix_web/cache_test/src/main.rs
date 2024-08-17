#![allow(unused)]

use actix_web::{get, web, App, HttpServer, Responder};
use std::sync::Mutex;

// Application 快取
struct AppState {
    app_name: String,
    counter: Mutex<i32> // 訪客人數
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // 取用 Application cache
    let counter = data.counter.lock().unwrap(); 
    format!("Hello {app_name}!\n訪客人數: {counter}") 
}

// async fn index() -> impl Responder {
    // "Hello world!"
// }

#[get("/update")]
async fn update_cache(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap(); 
    *counter += 1;
    format!("訪客人數: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 建立 Application cache
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
                counter: 0.into() // convert integer into Mutex<i32>
            }))
            // .service(web::scope("/app")
                    // .route("/", web::get().to(index)))
            .service(index)
            .service(update_cache)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
