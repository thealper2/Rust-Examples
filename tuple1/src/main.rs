#[allow(dead_code)]
#[allow(unused_variables)]

fn topla_ve_carp(x:i32, y:i32) -> (i32, i32) {
    (x+y, x*y)
}

fn main() {
    let x:i32 = 3;
    let y:i32 = 4;
    let sonuclar = topla_ve_carp(x, y);
    println!("Sonuclar: {:?}", sonuclar);
    println!("{0}+{1} = {2} ve ayrica {0}*{1} = {3}", x, y, sonuclar.0, sonuclar.1);

    let(toplam, carpim) = sonuclar;
    println!("Toplam: {} ve carpim: {}", toplam, carpim);

    let sonuclar2 = topla_ve_carp(4, 8);
    let combine = (sonuclar, sonuclar2);
    println!("Tum sonuclar: {:?}", combine);
    println!("Sonuncu eleman: {}", (combine.1).1);

    let elemanlar = (true, 23.1, -1i8);
    println!("Elemanlar: {:?}", elemanlar);

    let eleman = (12,);
    println!("Eleman: {:?}", eleman);
}
