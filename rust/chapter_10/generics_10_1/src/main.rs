fn main() {
    let number_list = vec![1,2,3,10,4,5];
    let mut largest = number_list[0];
    for number in number_list{
        if number > largest{
            largest = number;
        }

    }
    println!("the largest number is{}",largest);
    println!("Hello, world!");
}
