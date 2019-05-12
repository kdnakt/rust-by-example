struct Container(i32, i32);

trait Contains {
    type A;
    type B;

    fn contains(&self, &Self::A, &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    type A = i32;
    type B = i32;

    fn contains(&self, n1: &i32, n2: &i32) -> bool {
        (&self.0 == n1) && (&self.1 == n2)
    }

    fn first(&self) -> i32 { self.0 }

    fn last(&self) -> i32 { self.1 }
}

fn diff<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let n1 = 4;
    let n2 = 15;
    let container = Container(n1, n2);

    println!("Does container contain {} and {}: {}",
        &n1, &n2, container.contains(&n1, &n2));
    println!("First: {}", container.first());
    println!("Last: {}", container.last());
    println!("The difference is: {}", diff(&container));
}