fn print_string(s:String){
    println!("print the cotents of the string '{}'",s);
}
fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();    
    let s1 = String::from("initial contents");
    print_string(s1);
    print_string(s);
    //error
    //print_str(s);
    let hello = String::from("你好 rust语言!");
    print_string(hello);


    let mut s2 = String::from("foo");
    s2.push_str("bar");
    print_string(s2);

    let mut s3 = String::from("hello");
    let content = ",world!";
    s3.push_str(content);
    println!("{}",content);
    //print_string(s3);
    s3.push('1');
    print_string(s3);
}
