#![allow(unused)]

use candle_core::{Device, Tensor};

fn random_tensor_test(device: &Device) -> 
    Result<(), Box<dyn std::error::Error>> {

    let a = Tensor::randn(0f32, 1., (2, 3), &device)?;
    let b = Tensor::randn(0f32, 1., (2, 3), &device)?;

    let c = a.add(&b)?;
    println!("a+b={}", c);

    Ok(())
}

// 反矩陣
fn inverse(tensor1: &Tensor, device: &Device) -> 
    Result<Tensor, Box<dyn std::error::Error>> {
    
    let vec: Vec<f32> = tensor1.flatten(0, 1)?.to_vec1()?;
    // let m11: f32 = vec[0];
    // println!("m11：{}\n", m11);
    
    let determinant = vec[0] * vec[3] - vec[2] * vec[1];
    
    let x: Vec<f32> = vec![vec[3]/determinant, 0.0f32-vec[1]/determinant, 
            0.0f32-vec[2]/determinant, vec[0]/determinant];
    let tensor2 = Tensor::new(x, &device)?
        .reshape((2,2))?;
    Ok(tensor2)
}

// 內積、轉置
fn tensor_method_test(device: &Device) -> 
    Result<(), Box<dyn std::error::Error>> {
        
    let x = [1f32,2.,3.,4.,5.,6.];
    let tensor1 = Tensor::new(&x, &device)?
        .reshape((2,3))?;
    println!("tensor1：\n{}\n", tensor1);

    let tensor2 = Tensor::arange::<f32>(1., 7., &device)?
        .reshape((3,2))?;
    println!("tensor2：\n{}\n", tensor2);

    // 內積
    let mul = tensor2.matmul(&tensor1)?;
    println!("內積：\n{}\n", mul);

    // 轉置
    println!("tensor2 轉置矩陣：\n{}\n", tensor2.transpose(0, 1)?);

    Ok(())
}

// 聯立方程式求解
fn solve_equations(device: &Device) -> 
    Result<(), Box<dyn std::error::Error>> {
    
    // x+y=5
    // x-y=1
    let left_coef: [f32; 4] = [1.,1.,1., -1.];
    let right_coef: [f32; 2] = [5.,1.];
    let tensor1 = Tensor::new(&left_coef, &device)?
        .reshape((2,2))?;

    let tensor2 = Tensor::new(&right_coef, &device)?
        .reshape((2,1))?;

    let tensor3 = inverse(&tensor1, &device)?;
    println!("反矩陣：\n{}\n", tensor3);

    // output
    println!("x+y=5\nx-y=1\n答案：\n{}", 
        tensor3.matmul(&tensor2)?);

    Ok(())
}

fn main() {
    let device = Device::Cpu;
    // let device = Device::new_cuda(0).unwrap();
    random_tensor_test(&device).unwrap();
    tensor_method_test(&device).unwrap();
    solve_equations(&device).unwrap();
}
