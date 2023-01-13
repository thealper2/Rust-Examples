#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let mut vec = vec![2, 34, 5];
    for x in &vec {
        println!("Deger: {}", *x);
    }
    for x in vec.iter() {
        println!("Iteratif: {}", x);
    }
    for x in vec.iter().rev() {
        println!("Tersten okuma {}", x);
    }
    for x in vec.iter_mut() {
        *x += 4;
    }
    println!("Liste halinde {:?}", vec);

    let mut vec2 = vec![2, 6, 3];
    vec2.extend(vec);
    println!("{:?}", vec2);
}
