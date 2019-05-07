//use std::fmt;

#[derive(Debug)]
struct A;
#[derive(Debug)]
struct S(A);
#[derive(Debug)]
struct SGen<T>(T);


fn reg_fn(s: S) {
    println!("reg_fn: {:?}", s)
}

fn gen_spec_t(s: SGen<A>) {
    println!("gen_spec_t: {:?}", s)
}

fn gen_spec_i32(s: SGen<i32>) {
    println!("gen_spec_i32: {:?}", s);
}

fn generic<T>(_s: SGen<T>) {
    //println!("generic: {:?}", s);
}

fn main() {
    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    generic::<char>(SGen('a'));

    generic(SGen('c'));
}