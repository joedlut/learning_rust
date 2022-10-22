fn main() {
    let some_value = 0u8;
    match some_value{
        1 => println!("one!"),
        2 => println!("two!"),
        3 => println!("three!"),
        _ => println!("zero!"),
    }
    let some_u8_value = Some(3);
    if let Some(3) = some_u8_value{
        println!("three");
    }
}
