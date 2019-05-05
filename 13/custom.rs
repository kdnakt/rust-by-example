#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
    // without `--cfg some_condition`,
    // rustc throws error: cannot find function `conditional_function` in this scope
}