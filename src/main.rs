use rust_ffi_playground::bindings::{gcd, Pair};

fn main() {
    let ps = &mut Pair { n: 6, m: 3 };
    let a = unsafe { gcd(0, 0, ps) };

    println!("Hello, world! {}", a);
}
