#![allow(dead_code)]

struct Point {
    x: i32,
    y: i32,
}

// 記錄成員的位址，必須加標籤【'a】，註明生命週期
#[derive(Debug)]
struct PointRef<'a> {
    x: &'a mut i32,
    y: i32,
}

#[derive(Debug)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let mut p1 = Point {
        x: 1,
        y: 2,
    };
    
    // 修改位址指到的p1.x
    let r = PointRef { x: &mut p1.x, y: p1.y };
    *r.x = 5; // 修改p1.x
    println!("{:?}", r);
    println!("{}", p1.x);

    // 宣告一個物件p2，其中成員的值可以來自另一個物件p1
    let p1 = Point3d {
        x: 1,
        y: 2,
        z: 3,
    };
    
    let p2 = Point3d {
        x: 10,
        .. p1
    };
    println!("{:?}", p2);
    
    let mut p3 = Point3d {
        x: 15,
        y: p1.y,
        z: p1.z,
    };
    println!("{:?}", p3);
    p3.y = 5;
    println!("{:?}", p1);
    
    
}
