pub struct NewsArticle{
    pub headline:String,
    pub location:String,
    pub author:String,
    pub content:String,
}

pub struct Tweet{
    pub username:String,
    pub content:String,
    pub reply:bool,
    pub retweet:bool,
}

pub trait Summary{
    fn summarize_author(&self)->String;

    fn summarize(&self)->String{
        format!("Read more from {}...",self.summarize_author())
    }


}

 
impl Summary for NewsArticle{
    fn summarize_author(&self)->String{
        format!("{}",self.author)
    }
}


impl Summary for Tweet{
    fn summarize_author(&self)->String{
        format!("@{}",self.username)
    }
}

fn main() {
    println!("Hello, world!");
    let article = NewsArticle{
        headline : String::from("Penguins win the Stanley Cup Championship!"),
        location : String::from("Pittsburgh,PA,USA"),
        author : String::from("Iceburgh"),
        content : String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("News Article available!{}",article.summarize());

    let tweet = Tweet{
        username: String::from("horse_ebooks"),
        content:String::from("of course,as probably already know,people"),
        reply:false,
        retweet:false,
    };
    println!("1 new tweet:{}",tweet.summarize());

}
