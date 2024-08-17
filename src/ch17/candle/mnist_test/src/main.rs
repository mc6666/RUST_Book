#![allow(unused)]

/* the following bug caused by the versions of two candle crates conflict.
expected `candle_core::Tensor`, found `candle_core::tensor::Tensor`
candle_* 均需使用 github
*/

// This should reach 91.5% accuracy.
#[cfg(feature = "mkl")]
extern crate intel_mkl_src;

#[cfg(feature = "accelerate")]
extern crate accelerate_src;

use std::time::{Duration, Instant};

use clap::{Parser, ValueEnum};
use rand::prelude::*;
use candle_core::{Device, DType, Result, Tensor, D};
use candle_nn::{loss, ops, Conv2d, Linear, Module, 
        ModuleT, Optimizer, VarBuilder, VarMap};

const IMAGE_DIM: usize = 784;
const LABELS: usize = 10;

struct TrainingArgs {
    learning_rate: f64,
    load: Option<String>,
    save: Option<String>,
    epochs: usize,
}

#[derive(Debug)]
struct ConvNet {
    conv1: Conv2d,
    conv2: Conv2d,
    fc1: Linear,
    fc2: Linear,
    dropout: candle_nn::Dropout,
}

impl ConvNet {
    fn new(vs: VarBuilder) -> Result<Self> {
        let conv1 = candle_nn::conv2d(1, 32, 5, Default::default(), vs.pp("c1"))?;
        let conv2 = candle_nn::conv2d(32, 64, 5, Default::default(), vs.pp("c2"))?;
        let fc1 = candle_nn::linear(1024, 1024, vs.pp("fc1"))?;
        let fc2 = candle_nn::linear(1024, LABELS, vs.pp("fc2"))?;
        let dropout = candle_nn::Dropout::new(0.5);
        Ok(Self {
            conv1,
            conv2,
            fc1,
            fc2,
            dropout,
        })
    }

    fn forward(&self, xs: &Tensor, train: bool) -> Result<Tensor> {
        let (b_sz, _img_dim) = xs.dims2()?;
        let xs = xs
            .reshape((b_sz, 1, 28, 28))?
            .apply(&self.conv1)?
            .max_pool2d(2)?
            .apply(&self.conv2)?
            .max_pool2d(2)?
            .flatten_from(1)?
            .apply(&self.fc1)?
            .relu()?;
        self.dropout.forward_t(&xs, train)?.apply(&self.fc2)
    }
}

fn train_model(
    m: candle_datasets::vision::Dataset,
    args: &TrainingArgs,
) -> anyhow::Result<()> {
    const BSIZE: usize = 2000;

    let dev = Device::Cpu;

    let train_labels = m.train_labels;
    let train_images = m.train_images;
    let train_labels = train_labels.to_dtype(DType::U32)?;

    let mut varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &dev);
    let model = ConvNet::new(vs.clone())?;

    if let Some(load) = &args.load {
        println!("loading weights from {load}");
        varmap.load(load)?
    }

    let adamw_params = candle_nn::ParamsAdamW {
        lr: args.learning_rate,
        ..Default::default()
    };
    let mut opt = candle_nn::AdamW::new(varmap.all_vars(), adamw_params)?;
    let test_images = m.test_images.to_device(&dev)?;
    let test_labels = m.test_labels.to_dtype(DType::U32)?;
    let n_batches = train_images.dim(0)? / BSIZE;
    let mut batch_idxs = (0..n_batches).collect::<Vec<usize>>();
    for epoch in 1..args.epochs {
        let mut sum_loss = 0f32;
        batch_idxs.shuffle(&mut thread_rng());
        for batch_idx in batch_idxs.iter() {
            let train_images = train_images.narrow(0, batch_idx * BSIZE, BSIZE)?;
            let train_labels = train_labels.narrow(0, batch_idx * BSIZE, BSIZE)?;
            let logits = model.forward(&train_images, true)?;
            let log_sm = ops::log_softmax(&logits, D::Minus1)?;
            let loss = loss::nll(&log_sm, &train_labels)?;
            opt.backward_step(&loss)?;
            sum_loss += loss.to_vec0::<f32>()?;
        }
        let avg_loss = sum_loss / n_batches as f32;

        let test_logits = model.forward(&test_images, false)?;
        let sum_ok = test_logits
            .argmax(D::Minus1)?
            .eq(&test_labels)?
            .to_dtype(DType::F32)?
            .sum_all()?
            .to_scalar::<f32>()?;
        let test_accuracy = sum_ok / test_labels.dims1()? as f32;
        println!(
            "{epoch:4} train loss {:8.5} test acc: {:5.2}%",
            avg_loss,
            100. * test_accuracy
        );
    }
    if let Some(save) = &args.save {
        println!("saving trained weights in {save}");
        varmap.save(save)?
    }
    Ok(())
}

fn main() -> anyhow::Result<()> {
    // Load the dataset
    let m = candle_datasets::vision::mnist::load()?;
    println!("資料載入完成.");
    println!("train-images: {:?}", m.train_images.shape());
    println!("train-labels: {:?}", m.train_labels.shape());
    println!("test-images: {:?}", m.test_images.shape());
    println!("test-labels: {:?}", m.test_labels.shape());

    let start = Instant::now();
    
    // 訓練模型
    let training_args = TrainingArgs {
        epochs: 5,
        learning_rate: 0.001,
        load: None,
        save: Some(String::from("mnist.model")),
    };
    train_model(m, &training_args);

    let duration = start.elapsed();
    println!("訓練時間: {:?}", duration);

    Ok(())
}