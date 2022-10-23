fn main() {
    let mut v = vec![1,2,3,4,5,6,78];
    let first = &v[0];
    v.push(345);
    println!("hello world!");
    // error
    //println!("the first element is {}",first);
    for i in &v{
        println!("{}",i);
    }
    for j in &mut v{
        *j += 10 ;
    }
    for j in &v{
        println!("{}",j);
    }
}
