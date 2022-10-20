struct User{
    username:String,
    email:String,
    sign_in_account:u64,
    active:bool,
}

fn build_user(email:String,username:String)->User{
    User{
        email:email,
        username:username,
        active:true,
        sign_in_account:1,
    }
}
fn build_user_1(email:String,username:String)->User{
    User{
        email,
        username,
        active:true,
        sign_in_account:1,
    }
}

fn main() {
    let mut user = User{
    email:String::from("someone@example.com"),
    username:String::from("someuser123"),
    active:false,
    sign_in_account:1,
    };
    user.email = String::from("2532771263@qq.com");
    //println!("{}",user);
    let mut user1 = User{
    email:String::from("123@qq.com"),
    username:String::from("joedlut"),
    ..user
    };
}

