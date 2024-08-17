use polars::prelude::*;

fn main() {
    let csv_path = "./countries.csv";
    let df:DataFrame = LazyCsvReader::new(csv_path)
            .has_header(true).finish().unwrap().collect().unwrap(); 

    // 顯示前10筆資料
    // println!("{}", df.head(None));
    
    // 篩選
    let out = df.clone().lazy()
    .filter(
        col("Country").str().starts_with(lit("Saint"))
        .and(col("Region").eq(lit("NORTH AMERICA"))),
    )
    .collect().unwrap();
    println!("{}", out);
}
