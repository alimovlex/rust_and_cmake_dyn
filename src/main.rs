// https://internals.rust-lang.org/t/meaning-of-link-kinds/2686
#[link(name="sandbox", kind="dylib")]
extern {
    // this is rustified prototype of the function from our C library
    fn testcall(v: f32);
}

fn main() {
    println!("Hello, world from Rust!");
    // calling the function from foo library
    unsafe {
        testcall(std::f32::consts::PI);
    };
}