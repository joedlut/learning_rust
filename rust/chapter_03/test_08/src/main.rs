fn main() {
    let mut counter = 0 ;
    let result = {
        loop{
            counter += 1;
        if counter == 10 
        {
            break counter * 2
        }
        }
    };
    println!("the result is {}",result);

    let mut number = 3;
    while number != 0{
        println!("{}!!",number);
        number -= 1;
    }
    println!("off!");

    let a = [1,2,3,5,6,7,8];
    let mut index = 0;
    while index < 5{
        println!("the value is {}",a[index]);
        index += 1;
    }

    println!("------------------------");
    // recommended
    for element in a.iter(){
        println!("the value is {}",element);
    }

    println!("------------------------");
    for num in (1..4).rev(){
        println!("{}!",num);
    }
    println!("liftoff!!");
}
