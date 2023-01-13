#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let mut c = Vec::new();
    c.push(1);
    c.push(2);
    c.push(3);
    println!("{:?}", c);
    c.push(4);
    println!("{:?}", c);

    let mut a = vec![1, 2, 3, 4, 6, 1, 1];
    println!("{:?}", a);

    let index:usize = 4;
    println!("a[4] = {}", a[index]);

    match a.get(11) {
        Some(x) => println!("a[5] = {}", x),
        None => println!("Hata, deger yok.")
    }

    for x in &a {
        println!("{}", x);
    }

    a.push(55);
    println!("{:?}", a);

    let son_eleman = a.pop();
    println!("Son eleman: {:?} ve a vektoru: {:?}", son_eleman, a);

    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}
