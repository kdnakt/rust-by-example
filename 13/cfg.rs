#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!");
}

#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!");
}

#[cfg(target_os = "macos")]
fn are_you_on_macos() {
    println!("You are running macos!");
}

#[cfg(not(target_os = "macos"))]
fn are_you_on_macos() {
    println!("You are *not* running macos!");
}

fn main() {
    are_you_on_macos();
    are_you_on_linux();

    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}