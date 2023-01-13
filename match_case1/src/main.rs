#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let ulke_kodu:i32 = 46;
    let ulke:&str = match ulke_kodu {
        44 => "UK",
        46 => "Sweeden",
        7 => "Russia",
        90 => "Turkey",
        1..=1000 => "Unknown",
        _=> "invalid"
    };
    println!("The country code {} is {}", ulke_kodu, ulke);

    let x:bool = true;
    let deger:&str = match x {
        false => "Yanlis",
        true => "Dogru",
        _=> "Ongorulemeyen"
    };
    println!("{}", deger);
}
