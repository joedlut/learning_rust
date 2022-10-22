#[derive(Debug)]
enum IpAddrKind{
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr{
    kind:IpAddrKind,
    address:String,
}
#[derive(Debug)]
enum IpAddr1{
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum IpAddr2{
    V4(u8,u8,u8,u8),
    V6(String),
}

fn main() {
    let home = IpAddr{
    //must use namespace ::
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1"),
    };
    let loopback = IpAddr{
        kind:IpAddrKind::V6,
        address:String::from("::1"),
    };
    println!("{:?}",home);
    println!("{:?}",loopback);

    let home1 = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr1::V6(String::from("::1"));
    println!("{:?}",home1);
    println!("{:?}",loopback1);
    let home2 = IpAddr2::V4(127,0,0,1);
    let loopback2 = IpAddr2::V6(String::from("::1"));
    println!("{:?}",home2);
    println!("{:?}",loopback2);
}
