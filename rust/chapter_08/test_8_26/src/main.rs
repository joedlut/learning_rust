use std::collections::HashMap;
fn main() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        //return the &mut V
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);
}
