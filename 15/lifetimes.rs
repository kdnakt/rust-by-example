fn main() {
    let i = 3;// lifetime starts
    {
        let borrow1 = &i;// starts borrow1
        println!("borrow1: {}", borrow1);
    }// ends borrow1
    {
        let borrow2 = &i;// starts borrow2
        println!("borrow2: {}", borrow2);
    }// ends borrow2
}// lifetime ends