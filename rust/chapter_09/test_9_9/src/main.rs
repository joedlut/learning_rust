use std::io::Read;
use std::io;
use std::fs::File;

fn read_username_from_file()->Result<String,io::Error>{
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let name = match read_username_from_file(){
        Ok(s)=>s,
        Err(e)=>panic!("read name failed{:?}",e),
    };
    println!("{}",name);
}
