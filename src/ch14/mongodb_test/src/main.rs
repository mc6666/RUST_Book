#![allow(unused)]

use mongodb::{Client, options::ClientOptions, Cursor};
use mongodb::error::Error;
use mongodb::bson::Document;
use mongodb::{bson::doc, options::FindOptions};
use futures_util::TryStreamExt; // for cursor.try_next()

async fn list_database_names() -> Result<Client, Error> {
    // 使用本機的連線字串，未設帳號/密碼
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // 建立用戶端
    let client = Client::with_options(client_options)?;

    // 顯示所有資料庫名稱
    for db_name in client.list_database_names(None, None).await? {
        println!("{}", db_name);
    }
    Ok(client)
}

async fn list_collection_names(client:Client, db: &str) -> Result<bool, Error> {
    // 連線資料庫
    let db = client.database(db);

    // 顯示所有集合(collections)
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    Ok(true)
}

// 顯示所有文件(documents)
async fn list_document(client:Client, db: &str, collection: &str) -> Result<bool, Error> {
    // 連線資料庫
    let db = client.database(db);

    // 顯示某一集合(collections)的控制碼
    let collection = db.collection::<Document>(collection);

    // 篩選【年齡】 >= 20
    let filter = doc! { "Age": { "$gte": 20 } };
    // 按【Name】排序
    let find_options = FindOptions::builder().sort(doc! { "Name": 1 }).build();
    let mut cursor: Cursor<Document> = collection.find(filter, find_options).await?;

    // 顯示結果
    while let Some(doc) = cursor.try_next().await? {
        println!("{:?}", doc);
    }
    Ok(true)
}

async fn insert_document(client:Client, db: &str, collection: &str) -> Result<bool, Error> {
    // Get a handle to a database.
    let db = client.database(db);

    // Get a handle to a collection in the database.
    let collection = db.collection::<Document>(collection);

    // List the names of the collections in that database.
    let docs = vec![
        doc! { "Name": "John", "Age": 40 },
        doc! { "Name": "Mary", "Age": 60 },
        doc! { "Name": "Tome", "Age": 20 },
        doc! { "Name": "Allen", "Age": 10 },
    ];

    // Insert some documents into collection.
    collection.insert_many(docs, None).await?;

    Ok(true)
}

#[tokio::main]
async fn main() {
    let client = list_database_names().await.unwrap();
    println!("");
    list_collection_names(client.clone(), "test").await;
    println!("");
    insert_document(client.clone(), "test", "users").await;
    println!("");
    list_document(client, "test", "users").await;
}
