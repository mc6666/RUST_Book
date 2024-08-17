#[derive(Copy, Clone, Debug)]
struct Tire {
    no:i32
}

#[derive(Debug)]
struct Car {
	tires: [Tire; 4]
}

fn main() {
    let mut x = Car{ tires: [Tire{no:0}; 4] };
    
    // 修改1個 Tire no
    x.tires[1].no=1;
    println!("{:?}", x);
    
    // 修改多個 Tire no
    for (i, tire) in x.tires.iter_mut().enumerate() {
        tire.no = i as i32;
    }
    println!("{:?}", x);
    
    // NOT work
    // for j in [0..4] {
        // x.tires[j].no = j;
        // x.tires[j] = Tire{no:j};
    // }
    // println!("{:?}", x);
}