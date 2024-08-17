use sqlx::mysql::MySqlPool;
use sqlx::Row;  // for try_get
use std::fs::File;
use std::io::BufRead; // for BufReader
use futures_util::TryStreamExt;  // for try_next

#[tokio::main]
async fn main() {
    // 建立資料庫連線
    let pool = MySqlPool::connect("mysql://root:1234@localhost/testdb")
                .await.unwrap();
    let mut conn = pool.acquire().await.unwrap();
    
    // 刪除user資料表
    let sql = "DROP TABLE IF EXISTS user";
    sqlx::query(sql).execute(&mut *conn).await.unwrap();
    let sql = String::from_utf8(std::fs::read("create_table.sql").unwrap()).unwrap();
    sqlx::query(&sql).execute(&mut *conn).await.unwrap();
    
    // 讀取資料檔 data.txt
    let file = File::open("data.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    // 解析 data.txt，將資料新增至user資料表
    for line in reader.lines() {
        // 解析每一列資料
        let line = line.unwrap();
        let vec1:Vec<&str> = line.split(",").collect();
        let first_name = vec1[0].trim();
        let last_name = vec1[1].trim();
        let age:i32 = vec1[2].trim().parse().unwrap();
        
        // 新增至user資料表：bind可填入參數值
        sqlx::query("insert into user (first_name, last_name, age) values (?, ?, ?)")
                .bind(first_name).bind(last_name).bind(age)
                .execute(&mut *conn).await.unwrap();
    }
    
    // 查詢
    let mut rows = sqlx::query("SELECT * FROM user WHERE age >= ?")
    .bind(30)
    .fetch(&mut *conn);

    // 顯示查詢結果
    while let Some(row) = rows.try_next().await.unwrap() {
        let first_name: &str = row.try_get("first_name").unwrap();
        let age: i32 = row.try_get("age").unwrap();
        println!("first_name:{}, age:{}", first_name, age);
    }
}
