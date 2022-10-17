fn main() {
    // i8 i16 i32 i64 u8 u16 u32 u64
    // f32 f64
    // bool
    // char ''  ""
    // tuple array
    
    //tuple 
    let tup:(i32,f64,u8) = (500,12.4,1);
    let (x,y,z) = tup;
    println!("the value of y is {}",y);
    let one = tup.0;
    let two = tup.1;
    let three = tup.2;
    println!("{} {} {}",one,two,three);

    //array stack
    let a = [1,2,4,5,6];
    let months = ["January","February","March","April","May","June","JUly","August","September","October","November","December"];

    let b:[i32;5] = [1,2,3,4,5];
    let first = b[0];
    let second = b[1];
    println!("{}",first);
    println!("{}",second);
   
    let index = 10;
    //illegal
    //println!("{}",b[index]);
}
