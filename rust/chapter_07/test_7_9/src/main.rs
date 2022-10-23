mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }
    #[derive(Debug)]
    // enum pub all public
    pub enum Appetizer{
        Soup,
        Salad,
    }
    impl Breakfast{
       pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
       }
    }
}
pub fn eat_at_restaurant(){
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please",meal.toast);
    //error   seasonal_fruit  private field
   // meal.seasonal_fruit = String::from("blueberries");
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
    println!("{:?}",order1);
    println!("{:?}",order2);
}
fn main() {
    println!("Hello, world!");
    eat_at_restaurant();
}
