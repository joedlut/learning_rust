fn main() {
    let mut s = String::from("hello world");
    let r1 = &mut s;
    let r2 = &mut s;
   // println!("{},{}",r1,r2);
  // println!("{}",r1);
   println!("{}",r2);
}
