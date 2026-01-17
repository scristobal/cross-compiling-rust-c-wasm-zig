/// Some external C code
mod some_c {
    mod bindings {
        include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
    }

    pub fn gcd(n: i32, m: i32) -> i32 {
        let pair = &mut bindings::Pair { n, m };
        unsafe { bindings::gcd(pair) }
    }
}

fn main() {
    let a = some_c::gcd(9, 3);
    println!("Hello, world! {}", a);
}
