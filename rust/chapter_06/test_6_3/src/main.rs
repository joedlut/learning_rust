#[derive(Debug)]
enum Coin{
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}
#[derive(Debug)]
enum UsState{
    Alaska,
    Alabama,
}
fn value_in_cents(coin:&Coin)->u32{
    match coin{
        Coin::Penny=>1,
        Coin::Nickle=>5,
        Coin::Dime=>10,
        // add variable "state" here, therefore state can bind to value in UsState of Quarter(UsState)
        Coin::Quarter(state)=>{
            println!("State quarter from {:?}",state);
            25
        },
    }
}
fn value_in_coin(coin:&Coin)->u32{
    match coin{
        Coin::Penny =>{
            println!("Lucky penny");
            1
        },
        Coin::Nickle=>5,
        Coin::Dime=>10,
        Coin::Quarter(_)=>25,
    }
}
fn main() {
    let x:i8 = 5;
    let some_string = Some("a string");
    let some_number = Some(12);
    let y:Option<i8> = Some(23);
    //error
    //let sum = x + y;
    let quarter = &Coin::Quarter(UsState::Alabama);
    println!("{}",value_in_cents(quarter));
    let penny = &Coin::Penny;
    let penny1 = &Coin::Penny;

    let nickle = &Coin::Nickle;
    println!("{}",value_in_coin(penny));

    let mut count = 0;
    let coins = [quarter,penny,penny1,nickle];
    println!("{:?}",coins);
    for coin in coins.iter(){
        match coin{
            Coin::Quarter(state)=>{
                println!("State quarter from {:?}",state);
                println!("{}",25);
            },
            _=>count += 1,

        }
    }
    println!("there are {} coins where are not quarters",count);


}
