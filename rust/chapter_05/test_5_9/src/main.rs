fn area(dimensions:(u32,u32))->u32{
    dimensions.0 * dimensions.1
}
fn main() {
    let rect1 = (30,50);
    println!("the area is {}",area(rect1));
}
