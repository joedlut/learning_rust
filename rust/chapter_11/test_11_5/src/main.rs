#[derive(Debug)]
pub struct Rectangle{
    length:u32,
    width:u32,
}
impl Rectangle{
    pub fn can_hold(&self,other:&Rectangle)->bool{
        self.length > other.length && self.width > other.width
    }
}
fn main() {
    println!("Hello, world!");
    let r1 = Rectangle{length:10,width:5};
    let r2 = Rectangle{length:9,width:4};
    let r3 = Rectangle{length:11,width:3};
    let result = r1.can_hold(&r2);
    println!("{}",result);
    let result = r1.can_hold(&r3);
    println!("{}",result);
}
