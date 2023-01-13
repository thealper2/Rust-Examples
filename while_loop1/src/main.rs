#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let mut x:i32 = 1;
    while x < 1000 {
        x *= 2;
        if x == 128 {
            continue
        }
        println!("x = {}", x);
    }
    println!("Loop konusu");
    let mut y:i32 = 1;
    loop {
        y *= 2;
        println!("y = {}", y);
        if y == 1 << 10 {
            break;
        }
    }
}
