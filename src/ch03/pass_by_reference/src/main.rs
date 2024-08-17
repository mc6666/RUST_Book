fn update(arr:&mut [i32;3]){  // 加 &
   for i in 0..3 {
      arr[i] = 0;
   }
   println!("Inside update {:?}",arr);
}

fn main() {
   let mut arr = [10,20,30];
   update(&mut arr); // 加 &
   print!("Inside main {:?}",arr);
}