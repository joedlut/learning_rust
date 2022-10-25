pub fn greeting(str:&str)->String{
    format!("hello {}!",str)
} 
pub fn greeting1(name:&str)->String{
    format!("hello!")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
    #[test]
    fn greeting1_contains_name(){
        let result = greeting1("guodonghan");
        //assert!(result.contains("guodonghan"));
        assert!(result.contains("guodonghan"),"the greeting does not contain the name,the value is {}",result);
    }
}
