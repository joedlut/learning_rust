fn main() {
    let v = vec![1,2,3,4,5];
    let third:&i32 = &v[2];
    println!("the third element is {}",third);
    let first:i32 = v[0];
    println!("the first element is {}",first);
    println!("the first element is {}",first);
    match v.get(4){
        Some(fifth) => println!("the fifth element is {}",fifth),
        None => println!("there is no fifth element"),
    }
}
