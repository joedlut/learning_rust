use std::fs::File;
fn main() {
    //let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("failed to open hello.txt");
}
