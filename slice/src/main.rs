#[allow(dead_code)]
#[allow(unused_variables)]

use std::mem;

fn use_slice(slice:&mut[i32]) {
    println!("Yeni veri diliminin ilk elemani {}, ayrica eleman sayisi {}", slice[0], slice.len());
    slice[0] = 3333;
}

fn main() {
    let mut data = [1, 2, 3, 4, 5];
    use_slice(&mut data[1..4]);
    println!("{:?}", data);
}
