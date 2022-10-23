use std::io::Read;
use std::fs::File;
use std::io;
fn read_username_from_file()->Result<String,io::Error>{
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn main() {
    let s = match read_username_from_file(){
        Ok(s)=>s,
        Err(e)=>panic!("read name failed {:?}",e),
    };
    println!("the username :{}",s);
}
