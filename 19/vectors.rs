fn main() {
    let collected_iterator: Vec<i32> = (0..10).collect();
    // [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
    println!("Collected (0..10) into: {:?}", collected_iterator);

    let mut xs = vec![1i32, 2, 3];
    println!("Initial vector: {:?}", xs);

    println!("Push 4 into the vector");
    xs.push(4);
    println!("Vector: {:?}", xs);// [1, 2, 3, 4]

    //collected_iterator.push(0);// error: cannot borrow immutable local variable as mutable

    println!("Vector size: {}", xs.len());// 4

    println!("Second element: {}", xs[1]);// 2

    println!("Pop last element: {:?}", xs.pop());// Some(4)

    // panicked at index out of bounds: len is 3 but the index is 3
    //println!("Fourth element: {}", xs[3]);

    println!("Contents of xs:");
    for x in xs.iter() {
        println!("> {}", x);
    }

    for (i, x) in xs.iter().enumerate() {
        println!("In position {} we have value {}", i, x);
    }

    for x in xs.iter_mut() {
        *x *= 3;
    }
    println!("Updated vector: {:?}", xs);// [3, 6, 9]
}