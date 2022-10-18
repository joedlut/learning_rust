fn five()->i32{
    5
}
fn plus_one(x:i32)->i32{
    x + 1
}
fn main() {
    //let x = (let y = 6);
    //let x = {let y = 6};
    let x = {let y=6;y+1};
    println!("the value of x is {}",x);
    let z = five();
    println!("the value of z is {}",z);
    let y = plus_one(1000);
    println!("the value of y is {}",y);
}
