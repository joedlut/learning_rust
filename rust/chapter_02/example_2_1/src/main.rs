use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1,101);
    println!("please guess the number");
    loop{ 
        println!("please input the guess:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("you guessed:{}",guess);
        let guess:u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }    
}
