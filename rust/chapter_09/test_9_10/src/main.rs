use std::fs::File;
fn main() {
    //error
    let f = File::open("hello.txt")?;
}
