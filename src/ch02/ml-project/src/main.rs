// use std::fs::File;
// use std::io::Write;

use rand::rngs::SmallRng;
use rand::SeedableRng;

use linfa::prelude::*;
use linfa_trees::{DecisionTree, Result, SplitQuality};

fn main() -> Result<()> {
    let mut rng = SmallRng::seed_from_u64(42);

    // load Iris dataset
    println!("載入鳶尾花資料集");
    let (train, test) = linfa_datasets::iris()
        .shuffle(&mut rng)
        .split_with_ratio(0.8);

    println!("以決策樹演算法訓練模型");
    let gini_model = DecisionTree::params()
        .split_quality(SplitQuality::Gini)
        .max_depth(Some(100))
        .min_weight_split(1.0)
        .min_weight_leaf(1.0)
        .fit(&train)?;

    // 測試資料預測
    let gini_pred_y = gini_model.predict(&test);
    
    // 混淆矩陣
    let cm = gini_pred_y.confusion_matrix(&test)?;
    println!("混淆矩陣：{:?}", cm);

    println!(
        "準確率: {:.2}%",
        100.0 * cm.accuracy()
    );

    let feats = gini_model.features();
    println!("特徵重要性排序 {:?}", feats);

    // println!("Training model with entropy criterion ...");
    // let entropy_model = DecisionTree::params()
        // .split_quality(SplitQuality::Entropy)
        // .max_depth(Some(100))
        // .min_weight_split(10.0)
        // .min_weight_leaf(10.0)
        // .fit(&train)?;

    // let entropy_pred_y = entropy_model.predict(&test);
    // let cm = entropy_pred_y.confusion_matrix(&test)?;

    // println!("{:?}", cm);

    // println!(
        // "Test accuracy with Entropy criterion: {:.2}%",
        // 100.0 * cm.accuracy()
    // );

    // let feats = entropy_model.features();
    // println!("Features trained in this tree {:?}", feats);

    // let mut tikz = File::create("decision_tree_example.tex").unwrap();
    // tikz.write_all(
        // gini_model
            // .export_to_tikz()
            // .with_legend()
            // .to_string()
            // .as_bytes(),
    // )
    // .unwrap();
    // println!(" => generate Gini tree description with `latex decision_tree_example.tex`!");

    Ok(())
}
