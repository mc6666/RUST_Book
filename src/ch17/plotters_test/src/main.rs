#![allow(unused)]

use plotters::prelude::*;
use show_image::{create_window, ImageView, ImageInfo, event};
use image::io::Reader;
// mod svg;
// pub use svg::SVGBackend;

fn display_image(image_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 開啟檔案
    let rgb = Reader::open(image_path)?.decode()?.into_rgb8();
    // 取得圖檔寬度及高度
    let (width, height) = Reader::open(image_path)?.into_dimensions()?;
    // 轉換為 ImageView 格式
    let image = ImageView::new(ImageInfo::rgb8(width, height), &rgb);
    // 建立視窗
    let window = create_window("image", Default::default())?;
    // 視窗內顯示圖像
    window.set_image("image-001", image);
    
    // 按 Escape，視窗會關閉
    for event in window.event_channel().unwrap() {
      if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape) 
                        && event.input.state.is_pressed() {
                break;
            }
        }
    }
    Ok(())
}

#[show_image::main]
fn main() {
    // 定義輸出的圖檔名稱
    let image_path: &str = "./image.png";
    // 定義解析度
    let root_drawing_area = 
        BitMapBackend::new(image_path, (1024, 768))
        .into_drawing_area();

    // 定義白色背景
    root_drawing_area.fill(&WHITE).unwrap();

    // 定義座標軸範圍
    let mut chart = ChartBuilder::on(&root_drawing_area)
        .build_cartesian_2d(-3.14..3.14, -1.2..1.2)
        .unwrap();

    // 繪製線圖
    chart.draw_series(LineSeries::new(
        (-314..314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
        &RED
    )).unwrap();
    
    // 顯示圖形
    // root_drawing_area.present().unwrap();
    display_image(image_path);
}
