fn largest(list: &[i32])->i32{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![1,2,34,5,6];
    let result = largest(&number_list);
    println!("the largest number is {} ",result);
    let number_list = vec![23,45,6,7,345,12];
    let result = largest(&number_list);
    println!("the largest number is {}",result);
    println!("Hello, world!");
}
