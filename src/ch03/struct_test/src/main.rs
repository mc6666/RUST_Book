#![allow(dead_code)]

// Struct宣告
#[derive(Debug)]
struct Person {
  name: String,
  gender: char,
  age: u8
}

// 方法(Method)
impl Person {
    fn greeting(&self) {
        println!("嗨, 我是{}", self.name);
    }
}

fn main() {
    // 新增一個Person資料類型的物件
    let mut michael = Person {
      name: "小明".to_string(),
      gender: '男',
      age: 25
    };        
    println!("{michael:?}");
    
    // 另一種新增物件的寫法
    let name = "小美".to_string();
    let gender = '女';
    let age = 18;
    let mary = Person {
      name,
      gender,
      age
    };       
    println!("{mary:?}");
    
    // 欄位修改
    michael.age = 18;
    println!("{michael:?}");
    
    // 呼叫物件的方法
    michael.greeting();
}
