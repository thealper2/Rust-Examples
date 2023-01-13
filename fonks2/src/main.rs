#[allow(dead_code)]
#[allow(unused_variables)]

fn is_even(x:u32) -> bool {
    x % 2 == 0
}

fn buyuk_mu(limit:u32) -> impl Fn(u32) -> bool {
    move |y:u32| y > limit
}

fn main() {
    let limit:u32 = 500;
    let mut sum:u32 = 0;

    let limite_kadar = buyuk_mu(limit);
    for i in 0.. {
        let deger:u32 = i * i;
        if limite_kadar(deger) {
            break;
        } else if is_even(deger) {
            sum += deger;
        }
        println!("Dongunun toplami: {}", sum);
    }

    let toplam:u32 = (0..).map(|x| x*x).take_while(|&x| x < limit).filter(|x| is_even(*x)).fold(0, |sum, x| sum+x);
    println!("Fonksiyonlar ileri yontemle toplami: {}", toplam);
}