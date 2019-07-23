// extern crate we're testing, same as any other code would do
extern crate integration;

mod common;

#[test]
fn test_add() {
    common::setup();
    assert_eq!(integration::add(2, 3), 5);
}