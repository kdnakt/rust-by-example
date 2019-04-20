fn main() {
    let elem = 5u8;
    let mut vec = Vec::new();

    vec.push(elem);// commenting out -> error: cannot infer type for `T`
    vec.push(elem);

    println!("{:?}", vec);// [5, 5]
}