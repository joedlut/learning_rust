use std::collections::HashMap;
fn main() {
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name,field_value);
    //error
    //println!("{}",field_name);
    let item = "Favorite Color";
    let value = map.get(item);
    match value{
        Some(color)=>println!("the value is {}",color),
        None => println!("there is no value for key {}",item),
    }
    map.insert("Favorite Color".to_string(),"Red".to_string());
    println!("{:?}",map);
}
