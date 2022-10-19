fn no_dangle()->String{
    let s = String::from("hello world");
    s
}
fn main() {
    let s = no_dangle();
    println!("{}",s);
}
