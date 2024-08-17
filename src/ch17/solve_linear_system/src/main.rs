#![allow(unused)]
use nalgebra::{DMatrix, Matrix2, Matrix2x1, Matrix1x2};

fn main() -> 
    Result<(), Box<dyn std::error::Error>>  {
    
    let left_coef = Matrix2::new(1.,1.,1., -1.);
    let right_coef = DMatrix::from_vec(2, 1, vec![5.,1.]);
    let inv = left_coef.try_inverse().unwrap();
    println!("{inv}");
    // not work, nalgebra 計算為常數
    // let result = inv.dot(&right_coef);
    // println!("{result:?}");
    println!("{}", inv.transpose() * right_coef.clone());
    
    // 直接呼叫 solve
    let decomp = left_coef.lu();
    let result = decomp.solve(&right_coef).expect("Linear resolution failed.");
    println!("{result}");
    Ok(())
}
