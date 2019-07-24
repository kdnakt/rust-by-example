fn main() {
    let  raw_p: *const u32 = &10;

    // dereference of raw pointer is unsafe and requires unsafe function or block
    // raw pointers may be NULL, dangling or unaligned: they canviolate aliasing rules and cause data races: all of these are undefined behavior
    unsafe {
        assert!(*raw_p == 10);
    }
}