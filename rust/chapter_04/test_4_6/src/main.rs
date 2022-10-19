fn change(some_string:&mut String){
    some_string.push_str(",world");
}
fn main() {
    //let s1 = String::from("hello");
    //change(&s1);
    let mut  s1 = String::from("hello");
    println!("{}",s1);
    change(&mut s1);
    println!("{}",s1);
}

