#![allow(unused)]

use tch::Tensor;
use anyhow::Result;
use tch::{nn, nn::ModuleT, Device};
use std::env;
use tch::vision::{imagenet, resnet};

fn main() -> Result<()> {
    // 取得參數：要辨識的圖檔
    let args: Vec<String> = env::args().collect();
    let image_file = &args[1];
    
    // 縮放解析度為 224x224
    let image = imagenet::load_image_and_resize224(image_file)?;

    // 使用 CPU
    let mut vs = tch::nn::VarStore::new(tch::Device::Cpu);

    // 載入預先訓練的模型(Pretrained model)：resnet18
    let resnet18 = resnet::resnet18(&vs.root(), imagenet::CLASS_COUNT);
    // 載入 resnet18 權重檔
    // 下載：https://github.com/LaurentMazare/tch-rs/releases/download/mw/resnet18.ot
    let weight_file: &str = "./resnet18.ot";
    vs.load(weight_file)?;

    // 預測(辨識)
    let output = resnet18
        .forward_t(&image.unsqueeze(0), /*train=*/ false)
        .softmax(-1, tch::Kind::Float);

    // 顯示前5名的預測類別及機率
    for (probability, class) in imagenet::top(&output, 5).iter() {
        println!("{:50} {:5.2}%", class, 100.0 * probability)
    };
    Ok(())
}
