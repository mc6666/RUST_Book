#[macro_export]
macro_rules! my_vec {
    ( $( $x:expr ), *) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

macro_rules! find_min {
    ($x:expr) => ($x); // 先接收一個參數
    ($x:expr, $($y:expr),+) => ( // 再接收一個或多個參數
        // 遞迴
        std::cmp::min($x, find_min!($($y),+))
    )
}

fn main() {
    // my_vec test
    let vec = my_vec![1,2,3];
    println!("{:?}", vec);
    
    // find_min test
    println!("{}", find_min!(5, 2 * 3, 4));
}
