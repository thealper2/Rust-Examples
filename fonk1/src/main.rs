#[allow(dead_code)]
#[allow(unused_variables)]

fn temel_fonksiyon() {
    println!("Temel Fonksiyon");
}

fn deger_yaz(x:i32) {
    println!("Girdiginiz deger: {}", x);
}

fn deger_artir(x:&mut i32) {
    *x += 1;
}

fn carpma_islemi(x:i32, y:i32) -> i32 {
    println!("Carpma islemi sonucu");
    x*y
}

fn main() {
    temel_fonksiyon();
    deger_yaz(55);
    let mut z:i32 = 22;
    deger_artir(&mut z);
    println!("z degerinin artirilmis hali: {}", z);

    let a:i32 = 2;
    let b:i32 = 8;
    println!("{} * {} = {}", a, b, carpma_islemi(a, b));
}