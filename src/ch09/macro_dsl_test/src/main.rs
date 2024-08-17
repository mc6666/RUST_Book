macro_rules! calculate {
    (eval $e:expr) => {
        {
            let val: usize = $e; // 計算結果，並強制轉換資料型別
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    calculate! {
        eval 1 + 2 
    }

    calculate! {
        eval (1 + 2) * (3 / 4)
    }
}