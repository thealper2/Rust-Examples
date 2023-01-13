#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn main() {
    let mut a:[i32; 5] = [1, 2, 3, 4, 5];
    println!("a dizisi {} elemana sahiptir ve ilk elemani: {} dir.", a.len(), a[0]);

    a[0] = 22;
    println!("{}", a[0]);
    assert_eq!(a, [22, 2, 3, 4, 5]);

    if a!=[1,2,3,4,5] {
        println!("Diziler esit degildir.");
    }

    let b:[i32; 10] = [77; 10];
    for i in 0..b.len() {
        println!("{}. eleman: {}", i+1, b[i]);
    }
    println!("----------");
    println!("{:?}", b);
    println!("b dizisi {} byte boyutundadir.", mem::size_of_val(&b));
}
