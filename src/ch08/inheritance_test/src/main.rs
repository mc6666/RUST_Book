use rand::prelude::*;

struct Sheep {}
struct Cow {}

// 動物
trait Animal {
    // &'static str：&str(reference) 生命週期為全局變數
    fn noise(&self) -> &'static str;
}

//  為Sheep類別實作Animal介面
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

//  為Cow類別實作Animal介面
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// random_number < 0.5，回傳 Sheep，否則回傳 Cow
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    if random_number < 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}

// 測試
fn main() {
    let mut rng = thread_rng();
    for _ in 0..10 {
        let random_number: f64 = rng.gen();
        let animal = random_animal(random_number);
        println!("{}", animal.noise());
    }
}
