#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

use std::mem;

fn main() {
    let mut a:i32 = 2+5*3;
    println!("sonuc a: {}", a);
    a = a+1;
    a += 1;
    println!("{} / {} isleminden kalan = {}", a, 4, (a%4));

    let a_kupu:i32 = i32::pow(a, 3);
    println!("a degerinin kupu = {}", a_kupu);

    let b:f64 = 2.6;
    let b_kupu:f64 = f64::powi(b, 3);
    let b_ustu_pi:f64 = f64::powf(b, std::f64::consts::PI);
    println!("{0}, ustu 3 : {1} ve {0} ustu pi: {2}", b, b_kupu, b_ustu_pi);
}
