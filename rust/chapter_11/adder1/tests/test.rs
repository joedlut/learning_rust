use adder1;
mod common;
#[test]
fn it_works(){
    common::setup();
    assert_eq!(4,adder1::add_two(2));
}
