use std::fs::File;
fn main() {
    //error
    //let f:u32 = File::open("hello.txt");
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file)=>file,
        Err(error)=>{
            panic!("there was a problem opening the file{:?}",error)
        },
    };
}
