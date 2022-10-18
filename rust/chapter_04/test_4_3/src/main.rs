fn main() {
    let s = String::from("hello world");
    takes_ownership(s);
    //println!("{}",s);
    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string:String){
    println!("{}",some_string);
}

fn makes_copy(some_integer:u32){
    println!("{}",some_integer);
}
