struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    // deallocates both arguments
    fn double_drop(self, _: T) {}
}

fn main() {
    let empty = Empty;
    let null = Null;

    empty.double_drop(null);

    //empty;// error: use of moved value
    //null;// error: use of moved value
}