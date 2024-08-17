fn main() {
    // 1. 簡單枚舉
    #[derive(Debug)]
    enum Lunch{
        Chicken,
        Pork,
		Fish
    }

    let food = Lunch::Pork;
    println!("{:?}", food);
    
    // 2. 指定枚舉對應的數值
    #[derive(Debug)]
    enum Animal{
        Cat = 1,
        Dog = 2,
		Tiger = 3
    }

    let x = Animal::Dog;
    println!("{:?}", x);
   
    let val = Animal::Dog as isize;
    println!("{:?}", val);
   
    // 3. 指定枚舉對應的是資料類別
    enum Number {
        Int(i32),
        Float(f32),
    }
    
    // 指定類別的參數值
    let n: Number = Number::Int(10);
    
    // 比對
    match &n {
        &Number::Int(value) => println!("Integer {}", value),
        &Number::Float(value) => println!("Float {}", value),
    }
}
