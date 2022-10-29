#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn exploration(){
        assert_eq!(2+5,7);
    }
    #[test]
    fn another(){
        panic!("make this test fail");
    }
}
