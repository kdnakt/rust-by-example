use std::thread;

fn main() {
    let data = "8696789773741647185329 7327050364959
11861322575564723963297542 624962850
7085 6234701860851907960690014725639
38397966 707106094172783238747669219
523807952578 88236525459303330302837
58495327135744041 048897885734297812
699202164389808735488 08413720956532
162784246374525898603453748 28574668";

    let mut children = vec![];

    // Map phase
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);
        // without move, error: closure may outlive the current function, but it borrows i,
        // which is owned by the current function
        children.push(thread::spawn(move || -> u32 {
            let result = data_segment
                    .chars()
                    .map(|c| c.to_digit(10).expect("should be a digit"))
                    .sum();
            println!("processed segment {}, result={}", i, result);

            result
        }));
    }

    // Reduce phase
    let mut intermediate_sums = vec![];
    for child in children {
        let intermediate_sum = child.join().unwrap();
        intermediate_sums.push(intermediate_sum);
    }

    let final_result = intermediate_sums.iter().sum::<u32>();

    println!("Final sum result: {}", final_result);
}