fn main() {
    let number = 3;
    if number < 10 {
        println!("the condition is true");
    }else{
        println!("the condition is false");
    }

    //if number {
    //    println!("not ok");
    //}

    if number % 4 == 0 {
        println!("the number is divided by 4");
    }else if number % 3 == 0{
        println!("the number is divided by 3");
    }else if number % 2 == 0{
        println!("the number is divided by 2");
    }else{
        println!("the number is not divided by 4,3,2");
    }

    let condition = true;
    let number = if condition{
        45
    }else{
        54
    };
    println!("the value of number is {}",number);
    loop{
        println!("again!");
    }
    
}
