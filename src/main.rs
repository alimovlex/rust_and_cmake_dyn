// https://internals.rust-lang.org/t/meaning-of-link-kinds/2686
#[link(name="sandbox", kind="dylib")]
extern {
    // this is rustified prototype of the function from our C library
    fn characterSetTest();
    fn listFiles();
    fn pointersTest();
    fn fileTest();
    fn preprocessingTest();
    fn dataTypeSizeTest();
}

fn main() {
    println!("Initializing Sandbox_CLang from Rust!");
    // calling the function from foo library
    unsafe {
        characterSetTest();
        listFiles();
        pointersTest();
        fileTest();
        preprocessingTest();
        dataTypeSizeTest();
    };
}