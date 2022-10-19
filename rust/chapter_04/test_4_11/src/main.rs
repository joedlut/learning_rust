fn first_word(s:&String)->usize{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i 
        }
    }
    s.len()
}
fn main() {
    let mut s1 = String::from("hello world");
    let index = first_word(&s1);
    println!("{}",index);
    s1.clear();
}
