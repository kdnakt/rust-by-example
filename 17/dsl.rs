macro_rules! calculate {
    (hoge $e:expr) => {{
        {
            let val : usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    }};
}

fn main() {
    calculate! {
        hoge 1 * 2
    }

    calculate! {
        hoge (1 + 2) * (3 / 4)
    }

    calculate! {
        hoge 3 + 3 + 3 - 5
    }
}