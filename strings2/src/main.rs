#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let isim:&str = "Enes";
    let selamlama:String = format!("merhaba ben {}, iyi gunler", isim);
    println!("{}", selamlama);

    let selam:&str = "selam";
    let rust:&str = "Rust";
    let selam_rust:String = format!("{}, {}", selam, rust);
    println!("{}", selam_rust);

    let run:&str = "run";
    let forest:&str = "forest";
    let rfr:String = format!("{0}, {1}, {0}", run, forest);
    println!("{}", rfr);

    let info:String = format!("merhaba ben {son}.{ilk} {son}.", ilk="James", son="Bond");
    println!("{}", info);

    let mixed = format!("{1} {} {0} {} {veri}", "alfa", "beta", veri="delta");
    println!("{}", mixed);
}
