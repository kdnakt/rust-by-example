fn apply<F>(f: F) where
    F: FnOnce() {
    //F: Fn() {
    //F: FnMut() {// error closure `diary` implements `FnOnce` not `FnMut`
    f();
}

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    use std::mem;

    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);// captures by reference

        farewell.push_str("!!!");// forces `farewell` to be captured by mutable reference
        // so requires `FnMut`
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);// requires FnOnce, forces `farewell` to be captured by value
    };

    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));
}