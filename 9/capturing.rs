fn main() {
    use std::mem;

    let color = "green";

    let print = || println!("`color`: {}", color);

    // using the borrow
    print();
    print();

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    inc();

    // error: cannot borrow count as mutable more than once at a time
    //let _reborrow = &mut count;// second mutable borrow occurts here

    let movable = Box::new(3);
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
        // note: closure cannot be invoked more than once because it moves the variable movable out of its environment
    };

    consume();
    //consume();// error: use of moved value 


    let haystack = vec![1, 2, 3];
    // value moved into closure here
    let contains = move |needle| haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));
    //println!("{}", contains(2));// error: mismatched types expected &{integer}

    //error: use of moved value
    //println!("There're {} elements in vec", haystack.len());
    
    let haystack2 = vec![4, 5, 6];
    let contains2 = |n| haystack2.contains(n);

    println!("{}", contains2(&1));
    println!("{}", contains2(&4));

    // no `move` signature so haystack2 is still available
    println!("There're {} elements in vec2", haystack2.len());
}