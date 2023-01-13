#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

use std::mem;

struct Point {
    x:f64,
    y:f64
}

fn orjin() -> Point {
    Point{
        x:0.0,
        y:0.0
    }
}

fn main() {
    let p1 = orjin(); // Stack
    let p2 = Box::new(orjin()); // Heap

    println!("p1 boyut olarak {} yer isgal eder", mem::size_of_val(&p1)); // Stack
    println!("p2 boyut olarak {} yer isgal eder", mem::size_of_val(&p2)); // Heap

    let p3 = *p2;
    println!("deger: {}", p3.y);
}
