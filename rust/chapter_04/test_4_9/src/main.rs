fn dangle()->&String{
    let s = String::from("hello world");
    &s
}
fn main() {
    let dangle = dangle();
}
