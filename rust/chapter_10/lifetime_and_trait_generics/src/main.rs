use std::fmt::Display;
fn longest_with_an_announcement<'a,T>(x:&'a str,y:&'a str,ann:T)->&'a str
where T:Display
{
    println!("Announcement!{}",ann);
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
fn main() {
    println!("Hello, world!");
    let string1 = String::from("hello world");
    let string2 = String::from("hello world world");
    let result = longest_with_an_announcement(string1.as_str(),string2.as_str(),"hello hello");
    println!("{}",result);
}
