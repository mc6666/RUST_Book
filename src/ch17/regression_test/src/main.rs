#![allow(unused)]

use plotters::prelude::*;
use std::path::Path;
use polars::prelude::*;
use ndarray::prelude::*;
use ndarray::{array, Array, Array1, Array2, Array3, 
    ShapeBuilder, concatenate, Axis};
use ndarray_inverse::Inverse;

use linfa::traits::Fit;
use linfa::prelude::*;
use linfa_linear::LinearRegression;

fn read_csv(file_path: &str)  -> 
    Result<DataFrame, Box<dyn std::error::Error>> {
        
    let df = CsvReader::from_path(file_path)?
    .infer_schema(None)
    .has_header(true)
    .finish()?;
    Ok(df) 
}

fn gradient_descent(x: &Array2<f64>, y: &Array1<f64>, 
        learning_rate: f64, epochs: usize) -> Array1<f64> {
    let m = y.len() as f64;
    let mut weights = Array::zeros(x.ncols());
    for _ in 0..epochs {
        // let h = sigmoid(&x.dot(&weights));
        let h = &x.dot(&weights);
        let error = h - y;
        let gradient = x.t().dot(&error) / m; 
        weights = weights - (learning_rate * gradient);
        // println!("weights：{:?}", weights); 
        // 有錯誤，訓練過程SSE越來越大
        println!("SSE：{:?}", error.t().dot(&error)); 
    }
    weights
}

fn main() {
    // 讀取資料檔
    const FILE_PATH: &str = "./BostonHousing.csv";
    let df = read_csv(FILE_PATH).unwrap();
    println!("head：{}\n", df.head(Some(5)));

    // 計算平均數、標準差
    // let col_medv = &df["medv"]; // or df.column(medv)
    // println!("房價平均數：{:?}", &col_medv.mean().unwrap());
    // println!("房價標準差：{:?}\n", &col_medv.std(1).unwrap()); // 1:ddof
    
    // 將 DataFrame 轉換為 ndarray
    let mut x = df.to_ndarray::<Float64Type>(IndexOrder::Fortran).unwrap();
    // 分離 X、Y
    let y = x.slice(s![.., x.shape()[1]-1..]);
    let y2 = x.slice(s![.., x.shape()[1]-1]).to_owned();
    let x = x.slice(s![.., 0..x.shape()[1]-1]);
    
    // 以線性代數計算
    // y = wx + b*1, 合併[w, b]，故變數為[x, 1]
    let ones = Array2::<f64>::ones((x.shape()[0], 1));
    // println!("{:?}\n", ones); 
    let x2 = concatenate(Axis(1), &[ones.view(), x.view()]).unwrap();
    // println!("x：{:?}\ny：{:?}\n", x2.shape(), y.shape()); 
    // y=wx+b，計算權重(w)與偏差(b)
    let w = (x2.t().dot(&x2)).inv().unwrap().dot(&x2.t()).dot(&y);
    println!("w：{:?}\n", w); 
    
    // 以梯度下降法計算
    // let x: Array2<f64> = Array::from_shape_vec((5, 1), vec![1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    // let y: Array1<f64> = Array::from_vec(vec![1.1, 2.1, 3.1, 4.1, 5.1]);

    // y = wx + b*1, 合併[w, b]，故變數為[x, 1]
    // let mut x_with_intercept = Array::ones((x.nrows(), x.ncols() + 1));
    // x_with_intercept.slice_mut(s![.., 1..]).assign(&x);
    // println!("x：{:?}\ny：{:?}\n", x_with_intercept.shape(), y2.shape()); 

    // 標準化
    // x_with_intercept = x_with_intercept.clone() - x_with_intercept.mean_axis(Axis(0)).unwrap();
    // x_with_intercept = x_with_intercept.clone() / x_with_intercept.std_axis(Axis(0), 1.0);

    // let learning_rate: f64 = 0.01;
    // let epochs: usize = 10;
    // let weights = gradient_descent(&x_with_intercept, &y2, learning_rate, epochs);
    // println!("w：{:?}\n", weights); 
    // let weights = gradient_descent(&x, &y2, learning_rate, epochs);
    
    // 以 Linfa 套件驗證
    let dataset = Dataset::new(x.to_owned(), y2);
    let model = LinearRegression::default().fit(&dataset).unwrap();
    println!("model：{:?}\n", model); 
    
}
