struct User<'a>{
    username:&'a str,
    email:&'a str,
    sign_in_account:u64,
    active:bool,
}
fn main() {
    let user1 = User{
        email:"someone@example.com",
        username:"guodonghan",
        active:true,
        sign_in_account:123,
    };
}
