#[allow(dead_code)]
#[allow(unused_variables)]

fn merhaba_de() {
    println!("Selamlar");
}

fn main() {
    let selam = merhaba_de;
    selam();
    let arti_bir = |x:i32| -> i32 {x+1};
    let a:i32 = 8;
    println!("{} + 1 = {}", a, arti_bir(a));

    let mut iki:i32 = 2;
    {
        let arti_iki = |x| {
            let mut z=x;
            z += iki;
            z
        };
        println!("{} + 2 = {}", 22, arti_iki(22));
    }
    let odunc_al = &iki;
    println!("{}", odunc_al);

    let arti_uc = |mut x:i32| x += 3;
    let mut k:i32 = 14;
    arti_uc(k);
    println!("k = {}", k);
}
