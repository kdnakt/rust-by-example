fn main() {
    let mut _mutable_integer = 7i32;

    {
        let _large_integer = &_mutable_integer;// borrow
        // error: cannot assign to _mutable_integer because it is borrowed
        //_mutable_integer = 50;// assign
    }

    _mutable_integer = 3;
}