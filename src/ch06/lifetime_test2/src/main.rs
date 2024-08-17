#![allow(unused)]

// 使用struct定義屬性
struct Person<'a> {
    fname: &'a str,
    lname: &'a str
}
  impl<'a> Person<'a> {
      // new 不須宣告生命週期，因impl已宣告
      fn new(fname: &'a str, lname: &'a str) -> Person<'a> { 
          Person {
              fname : fname,
              lname : lname
          }
      }

      fn fullname(&self) -> String {
          format!("{} {}", self.fname , self.lname)
      }
  }

fn main() {
    let player = Person::new("Serena", "Williams");
    println!("Player: {}", player.fullname());
}