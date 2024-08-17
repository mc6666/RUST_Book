use rand::Rng;
use std::io;

fn main() {
    // 隨機產生一個數字，介於[0, 9]
    let mut rng = rand::thread_rng();
    let answer:u8 = rng.gen_range(0..10);
    
    let mut count = 0; // 猜錯的次數初始值=0
    'a1:loop {
        'a2:loop {
            let mut guess_no = String::new();
            println!("猜一個數字(0~9)：");
            io::stdin().read_line(&mut guess_no)
                .expect("Failed to read line"); // 例外控制
            if guess_no.trim() == "q" {
                println!("遊戲結束");
                break 'a1;
            }
                
            // 字串轉數字
            let guess_no:u8 = guess_no
                .trim()  // 去除首尾空白
                .parse() // 解析，將字串轉數值
                .expect("輸入不是數字."); // 例外控制
                
            if guess_no != answer {
                println!("猜錯了.");
                count += 1;
            } else {
                println!("猜對了.");
                break 'a2;
            }                     

            if count >= 5 {
                println!("失敗. 答案是{answer}");
                break 'a2;
            }
        }
        
        // 是否繼續玩
        let mut is_continue = String::new();
        count = 0;
        println!("是否繼續玩(y/n)：");
        io::stdin().read_line(&mut is_continue)
            .expect("Failed to read line"); // 例外控制
        if is_continue.trim() == "n" {
            println!("遊戲結束");
            break;
        }
    }
}
