use std::thread;

static NTHREADS: f32 = 10.0;

fn main() {
    let data = "8696789773741647185329 7327050364959
11861322575564723963297542 624962850
7085 6234701860851907960690014725639
38397966 707106094172783238747669219
523807952578 88236525459303330302837
58495327135744041 048897885734297812
699202164389808735488 08413720956532
162784246374525898603453748 285746681";

    println!("Number of threads: {}", NTHREADS);

    //let mut children = vec![];

    // // Map phase
    let chars: Vec<char> = data.chars()
            .filter(|c| c.is_numeric())
            .collect();
    let mut itr = chars.chunks(15);
    println!("it: {:?}", itr.next().unwrap());
    let mut nxt = itr.next();
    while !nxt.is_none() {
        println!("it: {:?}", nxt.unwrap());
        nxt = itr.next();
    }

    // for (i, data_segment) in chunked_data.enumerate() {
    //     println!("data segment {} is \"{}\"", i, data_segment);
    //     // without move, error: closure may outlive the current function, but it borrows i,
    //     // which is owned by the current function
    //     children.push(thread::spawn(move || -> u32 {
    //         let result = data_segment
    //                 .chars()
    //                 .map(|c| c.to_digit(10).expect("should be a digit"))
    //                 .sum();
    //         println!("processed segment {}, result={}", i, result);

    //         result
    //     }));
    // }

    // // Reduce phase
    // let mut intermediate_sums = vec![];
    // for child in children {
    //     let intermediate_sum = child.join().unwrap();
    //     intermediate_sums.push(intermediate_sum);
    // }

    //let final_result = intermediate_sums.iter().sum::<u32>();
    // without turbofish ::<>, explicitly specifying the type
    // let final_result: u32 = intermediate_sums.iter().sum();

    // println!("Final sum result: {}", final_result);
}