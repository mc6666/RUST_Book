use polars::prelude::*;

fn main() {
    let q = LazyCsvReader::new("./iris.csv")
        .has_header(true)
        .finish().unwrap()
        .filter(col("sepal_length").gt(lit(5)))
        .group_by(vec![col("species")])
        .agg([col("*").mean()]);

    let df = q.collect().unwrap();
    println!("{}", df);
}