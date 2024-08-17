fn main() {
    let mut i = 1;
    let mut sum = 0;
    while i <= 100 {
       sum+=i;
       i+=1;
    }
    println!("sum:{sum}");
    
    // while let
    let mut optional = Some(0);

    // 先執行 let，指定 optional = i，再判斷 optional == None?
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
}
