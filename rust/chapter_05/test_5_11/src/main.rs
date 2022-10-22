#[derive(Debug)]
struct Rectangle{
    width : u32,
    height: u32,
}
impl Rectangle{
    fn area(&self)->u32{
        self.width * self.height
    }
    fn width(&self)->bool{
        self.width >0
    }
    fn can_hold(&self,other:&Rectangle)->bool{
        self.width>other.width && self.height>other.height
    }
    //associated methods
    fn square(size:u32)->Self{
        Self{
            width:size,
            height:size,
        }
    }
}
fn main() {
    let rect1 = Rectangle{
        width:30,
        height:50,
    };
    dbg!(&rect1);
    println!("area is {}",rect1.area());
    println!("width is greater than 0 {}",rect1.width());
    println!("{}",rect1.width);
    let rect2 = Rectangle{
        width:10,
        height:45,
    };
    println!("rect1 can hold rect2 ? {}",rect1.can_hold(&rect2));
    let rect3 = Rectangle{
        width:31,
        height:23,
    };
    println!("rect1 can hold rect3 ? {}",rect1.can_hold(&rect3));
    let square1 = Rectangle::square(10);
    println!("{}",square1.area());
}

