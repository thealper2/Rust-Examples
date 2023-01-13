#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    let a:u8 = 125; // u = unsigned 0-255 8 bit
    println!("a = {}", a);

    // u unsigned, 0-2^N-1
    // i signed, -2^N - 2^N-1
    let mut b:i8 = 0;
    println!("once b = {}", b);
    b = 22;
    println!("sonra b = {}", b);

    let c:i32 = 123456789;
    println!("c = {} ve boyutu {} byte dir.", c, mem::size_of_val(&c));

    let d:isize = -200;
    let d_boyut = mem::size_of_val(&d);
    println!("d = {} ve boyutu {} byte, bilgisayariniz {} bit mimariye sahiptir.", d, d_boyut, d_boyut * 8); // 8 byte * 8 = 64 bit

    let e:char = 'a';
    println!("e = {} ve boyutu {} byte dir.", e, mem::size_of_val(&e));

    // f32 veya f64 u olamaz varsayilan olarak isaretli IEEE754 ile nan, +- sonsuz degerleri de alabilir.
    let f:f32 = 2.50002;
    println!("f = {} ve boyutu {} byte dir.", f, mem::size_of_val(&f));

    let g:bool = false;
    println!("g = {} ve boyutu {} byte dir.", g, mem::size_of_val(&g));
}