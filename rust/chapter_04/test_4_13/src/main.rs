// the slice type &str
fn first_word(s:&String)->&str{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn first_word_1(s:&str)->&str{
    let bytes = s.as_bytes();
    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    //let mut str = String::from("hello rust");
    let mut str = "hello world";
    //let word = first_word(&str);
    let word = first_word_1(str);
    println!("{}",word);
    //println!("{}",word);
    let mut str1 = String::from("hello world");
    // error
    //let word = first_word_1(str1);
    let word = first_word_1(&str1[..]);
    println!("{}",word);
}
