#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use Status::{Poor, Rich};
    use Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("The rich has lots of money!"),
        Poor => println!("The poor has no money ..."),
    }

    match work {
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
    }
}