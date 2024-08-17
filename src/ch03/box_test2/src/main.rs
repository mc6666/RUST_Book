use std::mem;

#[allow(dead_code)] // 不要出現警告訊息
#[derive(Debug, Clone, Copy)]
struct Point { // 點
    x: f64,
    y: f64,
}

#[allow(dead_code)]
struct Rectangle { // 矩形
    top_left: Point,
    bottom_right: Point,
}

// 原點，Point(0, 0)會放在 Stack
fn origin() -> Point { 
    Point { x: 0.0, y: 0.0 }
}

// Box原點，Point(0, 0)會放在 Heap
fn boxed_origin() -> Box<Point> { 
    Box::new(Point { x: 0.0, y: 0.0 })
}

// Box in a Box
struct Cons { 
    i: i32,
    b: Box<usize>
}


fn main() {
    // 變數值儲存在 Stack 區段
    let point: Point = origin();
    let rectangle: Rectangle = Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 }
    };

    // 矩形變數值儲存在 Heap 區段
    let boxed_rectangle: Box<Rectangle> = Box::new(Rectangle {
        top_left: origin(),
        bottom_right: Point { x: 3.0, y: -4.0 },
    });

    // 原點變數值儲存在 Heap 區段
    let boxed_point: Box<Point> = Box::new(origin());

    // 雙 Box
    let double_box: Box<Box<Point>> = Box::new(boxed_origin());
        

    // Stack 原點
    println!("原點佔 Stack {} bytes",
             mem::size_of_val(&point));
    println!("矩形佔 Stack {} bytes",
             mem::size_of_val(&rectangle));

    // box size == pointer size
    println!("Boxed 原點佔 Heap {} bytes",
             mem::size_of_val(&boxed_point));
    println!("Boxed 矩形佔 Heap {} bytes",
             mem::size_of_val(&boxed_rectangle));
    println!("雙 Box 原點佔 Heap {} bytes",
             mem::size_of_val(&double_box));
             
    // Box in a Box
    let con1 = Cons{i:2, b:Box::new(3)};
    let box_in_a_box:Box<Cons> = Box::new(con1);
    println!("像JSON表達式的 Box佔 Heap {} bytes",
             mem::size_of_val(&box_in_a_box));

    // 解除裝箱(deref)：利用*可取得箱內的值
    let unboxed_point: Point = *boxed_point;
    println!("Unboxed 原點佔 Stack {} bytes",
             mem::size_of_val(&unboxed_point));
}
