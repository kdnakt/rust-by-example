struct Point { x: i32, y: i32, z: i32 }

fn main() {
    let mut point = Point { x: 0, y: 0, z: 0 };
    {
        let borrowed_point = &point;
        let another_borrow = &point;
        println!("Point has coordinates: ({}, {}, {})",
            borrowed_point.x,
            another_borrow.y,
            point.z);
        // error: cannot borrow as mutable
        // because it is also borrowed as immutable
        //let mutable_borrow = &mut point;
    }
    {
        let mutable_borrow = &mut point;
        mutable_borrow.x = 5;
        mutable_borrow.y = 2;
        mutable_borrow.z = 1;

        // error: cannot borrow as immutable
        // because point is also borrowed as mutable
        //let y = &point.y;

        // error: cannot borrow as immutable
        // because point is also borrowed as mutable
        //println!("Point Z coordinate is {}", point.z);

        println!("Point has coordinates: ({}, {}, {})",
            mutable_borrow.x,
            mutable_borrow.y,
            mutable_borrow.z);
    }

    let borrowed_point = &point;
    println!("Point now has coorinates: ({}, {}, {})",// 5, 2, 1
        borrowed_point.x,
        borrowed_point.y,
        borrowed_point.z);
}