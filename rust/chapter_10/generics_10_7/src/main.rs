struct Point<T>{
        x:T,
        y:T,
}
fn main() {
    let wont_work = Point{x:5,y:10.0};
    println!("Hello, world!");
}
