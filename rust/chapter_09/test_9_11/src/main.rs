mod Guess_Game{
#[derive(Debug)]
pub struct Guess{
    value:i32,
}
impl Guess{
    pub fn new(value:i32)->Guess{
        if value<1 || value>100{
            panic!("Guess value must be between 1 and 100,got {}",value);
        }
        Guess{
            value
        }
    }
    pub fn value(&self)->i32{
        self.value
    }
    pub fn set_value(&mut self,value:i32){
        if value<1 || value>100{
            panic!("Guess value must be between 1 and 100,got {}",value);
        }
        self.value = value;
    }
}
}

fn main() {
    let mut guess = Guess_Game::Guess::new(20);
    println!("{:?}",guess);
    //println!("{}",guess.value());
    guess.set_value(190);
    println!("{:?}",guess);
}
