#![allow(unused)]

use std::path::Path;
use polars::prelude::*;

fn read_csv(file_path: &str)  -> 
    Result<DataFrame, Box<dyn std::error::Error>> {
        
    let df = CsvReader::from_path(file_path)?
    .infer_schema(None)
    .has_header(true)
    .finish()?;
    Ok(df)
}

fn main() {
    const FILE_PATH: &str = "./BostonHousing.csv";
    let df = read_csv(FILE_PATH).unwrap();
    println!("DataFrame：{}\n", df);
    println!("head：{}\n", df.head(Some(5)));
    println!("tail：{}\n", df.tail(Some(5)));
    
    // column selection
    let col_medv = &df["medv"]; // or df.column(medv)
    println!("column medv：{}\n", col_medv.head(Some(5)));
    
    // multiple column selection
    let multi_col = df.select(["zn", "medv"]).unwrap();
    println!("multi_col：{}\n", multi_col.head(Some(5)));
    
    // row selection, 自第2筆起，讀取5筆
    let rows = df.slice(2, 5);
    println!("rows：{}\n", rows);

    // 檢查 missing data
    println!("null count：{}\n", df.null_count());
    
    // 計算平均數、中位數、標準差
    println!("平均數：{:?}\n", col_medv.mean());
    println!("中位數：{:?}\n", col_medv.median());
    println!("標準差：{:?}\n", col_medv.std(1));
    println!("變異數：{:?}\n", col_medv.var(1));
    
    // 篩選
    let mask1 = col_medv.lt_eq(12.0).unwrap();
    let out = df
        .clone()
        .filter(&mask1).unwrap();
    println!("filter：{:?}", out);
    
    // 複合篩選
    let out = df
        .clone().lazy()
        .filter(col("medv").lt_eq(12.0).and(col("medv").gt_eq(10.0))
                ).collect().unwrap();
    println!("filter：{:?}", out);

    // 小計
    let out = df.clone().lazy() 
        .group_by(["chas"])
        .agg([mean("medv")]).collect().unwrap();
    println!("group_by：{:?}", out);

    // DataFrame 轉成 ndarray
    let out = df.to_ndarray::<Float64Type>(IndexOrder::Fortran);
    println!("ndarray：{:?}", out);
}
