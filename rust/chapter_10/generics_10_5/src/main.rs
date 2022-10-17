fn largest<T>(list:&[T])->T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![34,56,100,23,23];
    let result = largest(&number_list);
    println!("the largest number is {}",result);
    let char_list = vec!['a','z','b','q'];
    let result = largest(&char_list);
    println!("the largest char is {}",result);
    println!("Hello, world!");
}
