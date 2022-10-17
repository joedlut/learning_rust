use std::fmt::Display;

struct Pair<T>{
    x:T,
    y:T,
}


impl<T:Display + PartialOrd>Pair<T>{
    fn cmp_display(&self){
        if self.x >= self.y{
            println!("the largest number is {}",self.x);
        }else{
            println!("the largest number is {}",self.y);
        }
    }
}

fn main() {
    let p1 = Pair{
       x:10,
       y:78,
    };
    p1.cmp_display();
    println!("Hello, world!");
}
