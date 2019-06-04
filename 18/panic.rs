fn give_princess(gift: &str) {
    if gift == "snake" { panic!("AAAaaaaa!!!!"); }

    println!("I love {}s", gift);
}

fn main() {
    give_princess("teddy bear");
    give_princess("snake");
}