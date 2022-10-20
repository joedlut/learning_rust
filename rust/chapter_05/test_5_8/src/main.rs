fn area(width:u32,height:u32)->u32{
    width * height
}

fn main() {
    let width = 10;
    let height = 23;
    println!("the area of the rectangle is {} square pixels",area(width,height));
}
