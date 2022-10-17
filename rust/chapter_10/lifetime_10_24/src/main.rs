fn longest<'a>(x: &'a str,y: &'a str)->&'a str{
     if x.len()>y.len(){
        x
     }else{
        y
     }
}
fn main() {
    let string1 = String::from("xyz");
    let result;
    {
        let string2 = String::from("the long string is long");
        result = longest(string1.as_str(),string2.as_str());
    }
    println!("the longest string is {}",result);
}
