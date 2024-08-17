#![allow(unused)]

fn main() {
    // get
    let vec = vec!["john", "mary", "helen", "tom", "michael"];
    let vec2 = vec.get(2); // 第2個元素
    println!("{:?}", vec2.unwrap());
    let vec2 = vec.get(16); // 第16個元素
    println!("{:?}", vec2);
    let vec2 = vec.first(); // 第1個元素
    println!("{:?}", vec2.unwrap());
    let vec2 = vec.last(); // 最後1個元素
    println!("{:?}", vec2.unwrap());
    
    // skip + take
    let vec = vec!["john", "mary", "helen", "tom", "michael"];
    let vec2 = vec.into_iter().skip(1).take(2);
    for i in vec2 { println!("{}", i); }
    
    // contains：是否包含特定元素?
    let vec = vec![1, 2, 3, 4, 5];
    let vec2 = vec.contains(&3);
    println!("{}", vec2);
    
    // contains：字串是否包含特定元素?
    let vec = vec!["john", "mary", "helen", "tom", "michael"];
    let vec2 = vec.contains(&"mary");
    println!("{}", vec2);

    // repeat
    let mut vec = vec![1, 2, 3];
    let vec2 = vec.repeat(2);
    println!("{:?}", vec2);

    // split_at
    let vec = vec![1, 2, 3, 4, 5];
    let (left, right) = vec.split_at(2); // 從第3個元素切割
    println!("{:?}, {:?}", left, right);

    // concat string
    let vec = vec!["hello", " ", "world"];
    let vec2 = vec.concat();
    println!("{:?}", vec2);

    // concat array
    let vec = vec![[1, 2], [3, 4]];
    let vec2 = vec.concat();
    println!("{:?}", vec2);

}
