fn main() {
    // variable binding
    let x = 5;

    // expression;
    x;// warning: path statement with no effect
    x + 1;// warning: unused atirhmetic operation that must be used
    15;

    let x = 5u32;
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // this expression will be assigned to y
        x_cube + x_squared + x
    };

    let z = {
        2 * x;// the semicolon suppresses this expression and () is assined to z
    };

    println!("x is {:?}", x);// 5
    println!("y is {:?}", y);// 155
    println!("z is {:?}", z);// ()
}