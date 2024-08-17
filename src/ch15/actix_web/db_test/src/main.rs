#![allow(unused)]

mod schema;
mod models;
use diesel::prelude::*;
use diesel::debug_query;
use dotenvy::dotenv;
use std::env;
use diesel::{insert_into, select, update, delete};
use models::*;
use diesel::upsert::*;
use diesel::dsl::now;
use actix_web::{get, post, web, App, Responder, Result, 
                HttpServer, HttpResponse};
use actix_web::middleware::Logger;                
use schema::users::dsl::*;

// 建立資料庫連線
pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    
    // 讀取 .env
    let database_url = env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set");
    // 資料庫連線
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}"
        , database_url))
}

// 儲存
#[post("/update_user")]
async fn update_user(
    form: web::Form<User>
) -> impl Responder {
    let mut conn = establish_connection();
    let mut no = 0;
    // 新增
    if (form.id == -1) {
        no = insert_into(users)
            .values((name.eq(&form.name), email.eq(&form.email)))
            .execute(&mut conn).unwrap();
        // println!("no of inserted records:{}", no.unwrap());
    } else { // 修改
        no = update(users)
        .filter(id.eq(form.id))
        .set((name.eq(&form.name), email.eq(&form.email)))
        .execute(&mut conn).unwrap();
    }
    if (no > 0) {
        HttpResponse::Ok().body("")
    } else {
        HttpResponse::Ok().body("儲存失敗 !!")
    }
}

// 刪除
#[get("/delete_user/{user_id_string}")]
async fn delete_user(user_id_string: web::Path<String>) 
    -> impl Responder {
    let mut conn = establish_connection();
            
    let user_id = user_id_string.parse::<i32>().unwrap();
    let no = delete(users)
        .filter(id.eq(user_id))
        .execute(&mut conn).unwrap();   
    if (no > 0) {
        HttpResponse::Ok().body("")
    } else {
        HttpResponse::Ok().body("刪除失敗 !!")
    }
}

// 以 id 查詢記錄
#[get("/user/{id}")]
async fn get_user(user_id_string: web::Path<String>) 
    -> impl Responder {
    let mut conn = establish_connection();
            
    let user_id = user_id_string.parse::<i32>().unwrap();
    let mut user_vec: Vec<User> = users
        .filter(id.eq(user_id))
        .load(&mut conn)
        .expect("get user error");
    
    HttpResponse::Created().json(user_vec.pop())
}

// 查詢所有記錄
#[get("/get_user_list")]
async fn get_user_list() 
    -> impl Responder {
    let mut conn = establish_connection();
            
    let mut user_vec: Vec<User> = users
        .load(&mut conn)
        .expect("get user list error");
    // println!("{:?}", user_vec);
    let content = DataSource{data: user_vec};
    HttpResponse::Created().json(content)
}

// 顯示首頁
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body(include_str!("../static/index.html"))
}

// 主程式
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    println!("starting HTTP server at http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(get_user)
            .service(get_user_list)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}

