#![allow(unused)]

mod schema;
mod models;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::debug_query;
use dotenvy::dotenv;
use std::env;
use schema::posts::dsl::*;
use diesel::{insert_into, select, update, delete};
use models::*;
use diesel::upsert::*;
use diesel::dsl::now;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    
    // 讀取 .env
    let database_url = env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set");
    // 資料庫連線
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}"
        , database_url))
}

fn insert_tests() {
    let mut conn = establish_connection();
    
    // 單筆新增
    let no_of_inserted = insert_into(posts)
        .values((title.eq("Diesel 教學"), body.eq(
        "For this guide, we’re going to walk through CRUD,...")))
        .execute(&mut conn);
    println!("no of inserted records：{}", no_of_inserted.unwrap());

    // 顯示ORM產生的SQL
    let query = insert_into(posts)
        .values((title.eq("Diesel 教學"), body.eq(
        "For this guide, we’re going to walk through CRUD,...")))
        .returning(id); 
    let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);
    println!("query: {:?}", debug);
    
    // 多筆新增
    let no_of_inserted = insert_into(posts)
        .values(&vec![
        (title.eq("Diesel 教學"), body.eq("AAA")),
        (title.eq("Rust 教學"), body.eq("BBB")),
        ])
        .execute(&mut conn);
    println!("no of inserted records：{}", no_of_inserted.unwrap());
    
    // 單筆新增，取得新增記錄 id
    let inserted_records = insert_into(posts)
        .values((title.eq("Diesel 教學"), body.eq(
        "For this guide, we’re going to walk through CRUD,...")))
        .returning(id)
        .get_results::<i32>(&mut conn); // <i32>：type annotations
    println!("inserted records：{:?}", inserted_records.unwrap());
    
    // 多筆新增，取得新增記錄 id
    let new_posts = vec![
        NewPost { title: "Diesel 教學".to_string(), body: "AAA".to_string() },
        NewPost { title: "Rust 教學".to_string(), body: "BBB".to_string() },
    ];
    let inserted_records = insert_into(posts)
        .values(new_posts)
        .returning(id)
        .get_results::<i32>(&mut conn); // <i32>：type annotations
    println!("inserted records：{:?}", inserted_records.unwrap());
}

fn execute_sql() {
    let mut conn = establish_connection();
    diesel::sql_query(
        "CREATE UNIQUE INDEX users_name ON users (name)")
        .execute(&mut conn).unwrap();
}

fn update_tests() {
    use schema::users::dsl::*;
    let mut conn = establish_connection();
            
    update(users)
        .filter(updated_at.lt(now))
        .filter(name.eq("Tess"))
        .set(hair_color.eq("Gold"))
        .execute(&mut conn);
}

fn delete_tests() {
    let mut conn = establish_connection();
            
    delete(posts)
        .filter(body.eq("AAA"))
        .execute(&mut conn);
}

fn upsert_tests() {
    use schema::users::dsl::*;
    let mut conn = establish_connection();
    
    // 單筆新增或更正
    insert_into(users)
        .values((name.eq("Tess"), hair_color.eq("Brown")))
        .on_conflict_do_nothing() 
        .execute(&mut conn);
        
    insert_into(users)
        .values((name.eq("Tess"), hair_color.eq("Black")))
        .on_conflict(name)  // name重複
        .do_update()
        .set(hair_color.eq("Black"))
        .execute(&mut conn);
}

fn select_tests() {
    use schema::users::dsl::*;
    let mut conn = establish_connection();
            
    let results = posts
        .filter(title.eq("Diesel 教學"))
        .limit(5)
        .select(Post::as_select())
        .order_by(body.asc())
        .load(&mut conn)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.id);
        println!("{}", post.title);
        println!("{}", post.body);
        println!("-----------\n");
    }
}

fn main() {
    insert_tests();
    
    // 建立索引
    execute_sql();
    
    upsert_tests();
    
    update_tests();
    
    select_tests();
    
    delete_tests();
    select_tests();
}
