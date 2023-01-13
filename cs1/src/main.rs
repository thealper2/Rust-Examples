#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

use std::mem;

const VERI:i8 = 23;
static mut m:i8=22;

fn main() {
    let a:i32 = 56;
    {
        let a:i32 = 44;
        println!("dongunun ici a: {}", a);
    }
    println!("dongunun disi a: {}", a);
    println!("veri: {}", VERI);

    unsafe {
        m = 66;
        println!("m: {}", m);
    }
}
