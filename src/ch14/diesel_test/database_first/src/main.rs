#![allow(unused)]

mod schema;
mod models;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::debug_query;
use dotenvy::dotenv;
use std::env;
use schema::orders::dsl::*;
use schema::order_details::dsl::*;
use diesel::{insert_into, select, update, delete};
use models::*;
use diesel::upsert::*;
use diesel::dsl::now;
use chrono::prelude::*;
use chrono::{DateTime, Local, NaiveDate};
use diesel::result::Error;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    
    // 讀取 .env
    let database_url = env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set");
    // 資料庫連線
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}"
        , database_url))
}

fn transaction_tests() {
    let mut conn = establish_connection();
    
    // 新增一筆訂單
    // 表頭
    let new_order = Order{
        order_id: 30000,
        customer_id: Some("VINET".to_string()),
        employee_id: Some(5),
        order_date: Some(Local::now().date_naive()),
        required_date: NaiveDate::from_ymd_opt(2024, 7, 8),
        shipped_date: NaiveDate::from_ymd_opt(2024, 7, 31),
        ship_via: Some(3),
        freight: Some(32.0),
        ship_name: Some("Vins et alcools Chevalier".to_string()),
        ship_address: Some("中山路1號".to_string()),
        ship_city: Some("台北市".to_string()),
        ship_region: Some("中正區".to_string()),
        ship_postal_code: Some("105".to_string()),
        ship_country: Some("中華民國".to_string()),
    };
    
    // 表身
    let new_order_details = vec![
        OrderDetail { 
            order_id: 30000,
            product_id: 11,
            unit_price: 15.0,
            quantity: 5,
            discount: 0.0,
        },
        OrderDetail { 
            order_id: 30000,
            product_id: 22,
            unit_price: 18.0,
            quantity: 20,
            discount: 0.15,
        },
    ];
    
    
    conn.transaction::<(), _, _>(|conn| {
        let inserted_records = insert_into(orders)
            .values(&new_order)
            .execute(conn); 

        let inserted_records = insert_into(order_details)
            .values(&new_order_details)
            .execute(conn); 
        println!("insert OK !!");
        Ok::<_, Error>(())
        // diesel::result::QueryResult::Ok(())
    });
        
}

fn inner_join_tests() {
    let mut conn = establish_connection();
            
    let results = order_details
    .inner_join(orders)
    .filter(schema::orders::dsl::order_id.eq(10248))
    .select((OrderDetail::as_select(), Order::as_select()))
    .load::<(OrderDetail, Order)>(&mut conn)
    .expect("Error loading posts");

    // println!("Order:\n{:?}", results);
    for record in results {
        println!("{:?}", record);
        println!("-----------\n");
    }
}

/* 
fn orm_test_1() {
    let mut conn = establish_connection();
    
    // get order
    let order_selected = orders
        .filter(schema::orders::dsl::order_id.eq(10248))
        .select(Order::as_select())
        .get_result(&mut conn);

    // get order details
    let order_details_selected = 
        OrderDetail::belonging_to(&order_selected)
        .select(OrderDetail::as_select())
        .load(&mut conn);

    println!("Order:\n{:?}", order_selected);
    println!("-----------");
    for record in order_details_selected {
        println!("{:?}", record);
    }
} 
*/

fn main() {
    // transaction_tests();
    inner_join_tests();
    // orm_test_1();
}
