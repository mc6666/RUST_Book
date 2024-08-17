#![allow(unused)]

mod entities;
use entities::{prelude::*, *};

use sea_orm::*;
use crate::sea_query::SimpleExpr;
use dotenvy::dotenv;
use std::env;
use async_std::task;

// 建立資料庫連線
async fn get_connection(database_url: String) 
            -> Result<DatabaseConnection, DbErr> {
    let db = Database::connect(&database_url).await?;
    Ok(db)
}

// 新增
async fn insert_tests(db: &DatabaseConnection) 
            -> Result<InsertResult<entities::bakery::ActiveModel>, DbErr> {
    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()), // 設定name欄位值
        profit_margin: ActiveValue::Set(0.0), // 設定profit_margin欄位值
        ..Default::default() // 其餘欄位採預設值
    };
    let res = Bakery::insert(happy_bakery).exec(db).await?;
    Ok(res)
}

// 更正
async fn update_tests(db: &DatabaseConnection, id: i32) 
            -> Result<InsertResult<entities::chef::ActiveModel>, DbErr> {
    // 更正 bakery            
    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    let res = sad_bakery.update(db).await?;

    // 新增 bakery 所屬的 chef
    let john = chef::ActiveModel {
        name: ActiveValue::Set("John".to_owned()),
        bakery_id: ActiveValue::Set(id),
        ..Default::default()
    };
    let res = Chef::insert(john).exec(db).await?;
    Ok(res)
}

// 刪除
async fn delete_tests(db: &DatabaseConnection, id: i32) 
            -> Result<(), DbErr> {
    let res: DeleteResult = Chef::delete_many()
        .filter(chef::Column::BakeryId.eq(id))
        .exec(db)
        .await?;

    let bakery_record = bakery::ActiveModel {
        id: ActiveValue::Set(id), // The primary must be set
        ..Default::default()
    };
    let res = bakery_record.delete(db).await?;
    Ok(())
}

// 查詢所有記錄
async fn query_all_tests(db: &DatabaseConnection) 
            -> Result<Vec<bakery::Model>, DbErr> {
    let res: Vec<bakery::Model> = Bakery::find().all(db).await?;
    Ok(res)
}

// 以主鍵(PK)查詢
async fn query_with_key_tests(db: &DatabaseConnection, id: i32) 
            -> Result<Option<bakery::Model>, DbErr> {
    let res = Bakery::find_by_id(id).one(db).await?;
    Ok(res)
}

// 以條件查詢
async fn query_with_filter_tests(db: &DatabaseConnection, filter: SimpleExpr) 
            -> Result<Option<bakery::Model>, DbErr> {
    let res = Bakery::find()
        .filter(filter)
        .one(db)
        .await?;
    Ok(res)
}


fn main() {
    // 自.env取得資料庫連線字串
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
                    .expect("DATABASE_URL must be set");
    // println!("database_url:{database_url}");
    
    // 建立資料庫連線
    let db = task::block_on(get_connection(database_url)).unwrap();
    
    // 新增 
    let result = task::block_on(insert_tests(&db)).unwrap();
    println!("inserted result:{result:?}");
    
    // 更正
    let result = task::block_on(update_tests(&db, result.last_insert_id)).unwrap();
    println!("inserted chef result:{result:?}");
    
    // 查詢所有記錄
    let result = task::block_on(query_all_tests(&db)).unwrap();
    println!("查詢所有記錄:");
    for record in result {
        println!("{record:?}");
    }
    
    // 以主鍵(PK)查詢
    let result = task::block_on(query_with_key_tests(&db, 2)).unwrap();
    if result.is_some() {
        println!("\n以主鍵(PK)查詢:{:?}", result.unwrap());
    }
    
    // 以條件查詢
    let result = task::block_on(query_with_filter_tests(&db, 
                bakery::Column::Name.eq("Sad Bakery"))).unwrap();
    println!("\n以條件查詢:{:?}", result.unwrap());
    
    // 刪除
    let result = task::block_on(delete_tests(&db, 4)).unwrap();

    // 查詢所有記錄
    let result = task::block_on(query_all_tests(&db)).unwrap();
    println!("\n刪除記錄後，查詢所有記錄:");
    for record in result {
        println!("{record:?}");
    }
}
