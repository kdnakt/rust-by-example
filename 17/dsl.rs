macro_rules! calculate {
    (foo $e:expr) => {{
        {
            let val : usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
    (foobar $e:expr) => {{
        {
            let val : usize = $e;
            println!("{} != {}", stringify!{$e}, val * val);
        }
    }};
}

fn main() {
    calculate! {
        foo 1 * 2
    }

    calculate! {
        foo (1 + 2) * (3 / 4)
    }

    calculate! {
        foobar 3 + 3 + 3 - 5
    }
}