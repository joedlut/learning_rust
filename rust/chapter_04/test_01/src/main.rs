fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // move  string   s1 -----> s2  s1 is not usable
    //println!("{},world",s1);
    
    let a1 = 10;
    let a2 = a1;
    println!("{}",a1);
    println!("{}",a2);


    let s3 = String::from("hello world");
    let s4 = s3.clone();
    println!("{}",s3);
    println!("{}",s4);
}
