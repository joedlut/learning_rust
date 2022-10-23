fn print_string(s:String){
    println!("{}",s);
}
fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    //fn add(self,s:&str)->String{...} s1 lost ownership
    let s3 = s1 + &s2;
    println!("{}",s3);
    println!("{}",s3);
    println!("{}",s2);
    //error
    //println!("{}",s1);
    let s4 = String::from("helo");
    println!("{}",s4);
    println!("{}",s4);
    //print_string(s4);
    //error
    //print_string(s4);
    print_string(s3);
    //print_string(s3);
}
