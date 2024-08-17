#![allow(unused)]

use anyhow::Error;
use odbc_api::{buffers::TextRowSet, Cursor, Environment, ConnectionOptions, 
               Connection, ResultSetMetadata, IntoParameter};
use std::{
    ffi::CStr,
    io::{stdout, Write},
    path::PathBuf,
};

// 每一批讀取資料的筆數
const BATCH_SIZE: usize = 5000;

// 資料表欄位
struct UserRecord {
    first_name: String,
    last_name: String,
    age: i32 
}    


// 查詢資料庫
fn query(connection: &Connection, sql: &str) -> Result<(), Error> {
    // 輸出至螢幕(標準輸出)
    let out = stdout();
    let mut writer = csv::Writer::from_writer(out);

    match connection.execute(sql, ())? {
        Some(mut cursor) => {
            // 將欄位名稱顯示在螢幕上
            let mut headline : Vec<String> = cursor.column_names()?.collect::<Result<_,_>>()?;
            writer.write_record(headline)?;

            // 設定欄位最大長度為 4KB
            let mut buffers = TextRowSet::for_cursor(BATCH_SIZE, &mut cursor, Some(4096))?;
            // 使用cursor，將資料填入變數內
            let mut row_set_cursor = cursor.bind_buffer(&mut buffers)?;

            // 使用 cursor 讀取每一批資料
            while let Some(batch) = row_set_cursor.fetch()? {
                // 讀取每一筆資料
                for row_index in 0..batch.num_rows() {
                    let record = (0..batch.num_cols()).map(|col_index| {
                        batch
                            .at(col_index, row_index) // 讀取一個欄位
                            .unwrap_or(&[]) // 讀不到資料時以空陣列([])表示
                    });
                    // 寫入標準輸出
                    writer.write_record(record)?;
                }
            }
        }
        None => {
            eprintln!(
                "無資料 !!"
            );
        }
    }

    Ok(())
}

fn insert(conn: &Connection, record: UserRecord) -> Result<(), Error>{   
    conn.execute(
        "INSERT INTO users VALUES (?, ?, ?)",
        (&record.first_name.into_parameter(), 
         &record.last_name.into_parameter(), 
         &record.age)
    )?;
    Ok(())
}

fn update(conn: &Connection, record: UserRecord) -> Result<(), Error>{   
    conn.execute(
        "UPDATE users SET first_name = ?, last_name= ?, age= ? where first_name = ?",
        (&record.first_name.clone().into_parameter(), 
         &record.last_name.into_parameter(), 
         &record.age,
         &record.first_name.into_parameter()
         )
    )?;
    Ok(())
}

fn delete(conn: &Connection, first_name: &str) -> Result<(), Error>{   
    conn.execute(
        "DELETE FROM users Where first_name = ?",
        (&first_name.into_parameter())
    )?;
    Ok(())
}


fn main() {
    // 建立環境
    let environment: Environment = Environment::new().unwrap();

    // 資料庫連線字串
    let connection_string = "
        Driver={ODBC Driver 17 for SQL Server};\
        Server=localhost;Database=testdb;UID=test;PWD=1234;";
    
    // 資料庫連線
    let connection = environment.connect_with_connection_string(
                connection_string, ConnectionOptions::default()).unwrap();

    // Connect using DSN
    // let connection = environment.connect(
        // "DataSourceName",
        // "Username",
        // "Password",
        // ConnectionOptions::default(),
    // )?;
    
    // insert
    let record = UserRecord {
        first_name: "Tom".to_string(),
        last_name: "Chen".to_string(),
        age: 27
    };
    insert(&connection, record); 
    
    let record = UserRecord {
        first_name: "Tom".to_string(),
        last_name: "Wang".to_string(),
        age: 30
    };
    update(&connection, record); 

    query(&connection, "SELECT * FROM users");
    
    delete(&connection, "Tom"); 
}
