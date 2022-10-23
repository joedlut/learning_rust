fn main() {
    let v:Vec<i32> = Vec::new();
    let v = vec![1,2,3];
    let mut v1 = Vec::new();
    v1.push(5);
    v1.push(6);
    {
        let mut v2 = Vec::new();
        v2.push(123);
        v2.push(232323232);
        println!("{:?}",v2);
    }
}
