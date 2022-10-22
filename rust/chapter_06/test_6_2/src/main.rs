#[derive(Debug)]
enum Message{
    Quit,
    Move{x:i32,y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}
impl Message{
    fn call(&self){
       println!("{:?}",self);
    }
}
//
struct QuitMessage;
struct MoveMessage{
    x:i32,
    y:i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32,i32,i32);
fn main() {
    println!("hello world!");
    let m = Message::Write(String::from("hello"));
    m.call();
}
