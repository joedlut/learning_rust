#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

fn area(rectangle: &Rectangle)->u32{
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle{width:30,height:50};
    println!("the area is {}",area(&rect1));
    println!("{:?}",rect1);
    let scale = 2;
    let rect2 = Rectangle{
        width:30,
        height:dbg!(30*scale),
    };
    println!("{:?}",rect2);
    dbg!(&rect2);
}
