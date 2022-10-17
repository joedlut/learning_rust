fn largest_i32(list: &[i32])->i32{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn largest_char(list:&[char])->char{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
        largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34,50,25,1000,23];
    println!("the largest number is {}",largest_i32(&number_list));
    let char_list = vec!['y','z','a','e','b'];
    println!("the largest char is {}",largest_char(&char_list));
    println!("Hello, world!");
}
