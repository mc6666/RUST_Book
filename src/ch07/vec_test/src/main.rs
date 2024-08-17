fn main() {
    // 混合資料型別 1
    let vec = (34, 50.0, "25", '0', 65);
    println!("{:?}", vec);
    
    // 混合資料型別 2
    let vec: (i32, f64, &str, char, i32) = (34, 50.0, "25", '0', 65);
    println!("{:?}", vec);
    
    // 陣列資料型別 1
    let vec = [34.0, 50.0, 25 as f32, 0 as f32, 65 as f32];
    println!("{:?}", vec);
    
    // 陣列資料型別 2
    let vec: [f32;5] = [34.0, 50.0, 25., 0., 65.];
    println!("{:?}", vec);
    
    // 陣列資料型別 3
    let vec: Vec<f32> = vec![34.0, 50.0, 25., 0., 65.];
    println!("{:?}", vec);
    
    // 找最大值：整數
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("The largest number is {}", largest);

}
