use std::collections::HashSet;

fn main() {
    let mut a: HashSet<i32> = vec!(1i32, 2, 3).into_iter().collect();
    let mut b: HashSet<i32> = vec!(2i32, 3, 4).into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // panicked at Value 4 is already in set B!
    //assert!(b.insert(4), "Value 4 is already in set B!");

    b.insert(5);

    println!("A: {:?}", a);// {2, 3, 1, 4} or {3, 4, 2, 1} ... at random
    println!("B: {:?}", b);// {3, 2, 4, 5}

    // [2, 3, 1, 4, 5]
    println!("Union: {:?}", a.union(&b).collect::<Vec<&i32>>());

    // [1]
    println!("Difference: {:?}", a.difference(&b).collect::<Vec<&i32>>());

    // [2, 3, 4]
    println!("Intersection: {:?}", a.intersection(&b).collect::<Vec<&i32>>());

    // [1, 5]
    println!("Symmetric Difference: {:?}",
        a.symmetric_difference(&b).collect::<Vec<&i32>>());
}