fn plus_one(x:Option<i32>)->Option<i32>{
    match x {
        None=>None,
        Some(i)=>Some(i+1),
    }
}

fn plus_two(x:Option<i32>)->i32{
    match x {
        Some(i)=>i+2,
        None=>0,
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}",five);
    println!("{:?}",six);
    let seven = plus_two(five);
    println!("{}",seven);
}
