fn another_function(){
    println!("another function");
}

fn another_function_01(x:i32){
    println!("the value of x is {}",x);
}
fn another_function_02(x:i32,y:f64){
    println!("the value of x is {}",x);
    println!("the value of y is {}",y);
}
fn main() {
    println!("Hello, world!");
    another_function();
    another_function_01(10);
    another_function_02(100,100.1);
}
