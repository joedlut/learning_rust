fn largest<T>(list:&[T])->T
where T:PartialOrd + Copy
{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}
fn main() {
    println!("Hello, world!");
    let number_list = vec![34,23,12,100,45,12];
    let result = largest(&number_list);
    println!("The largest number is {}",result);
    let char_list = vec!['d','a','z','c','o'];
    let result = largest(&char_list);
    println!("The largest char is {}",result);
    println!("The largest char is {}",result);
}
