struct Point<T,U>{
    x:T,
    y:U,
}

fn main(){
    let both_integer = Point{x:5,y:10};
    let both_float = Point{x:10.0,y:90.0};
    let interger_and_float = Point{x:5,y:4.0};
    println!("hello world!");
}
