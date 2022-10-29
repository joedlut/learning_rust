pub fn add_two(a:i32)->i32{
    a + 2
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn add_two_test(){
        assert_eq!(4,add_two(2));
    }
    #[test]
    fn add_two_test1(){
        assert_eq!(add_two(3),5);
    }
    #[test]
    fn one_hundred(){
        assert_eq!(104,add_two(102));
    }
    #[test]
    #[ignore]
    fn expensive_test(){
        println!("this test will take an hour");
    }
}
