#[allow(dead_code)]
#[allow(unused_variables)]


use std::io::stdin;

fn main() {
    let mut veri = HashSet::new();
    veri.insert("gama");
    veri.insert("delta");
    println!("{:?}", veri);

    veri.insert("delta");
    println!("{:?}", veri);

    let eklendi_mi:bool = veri.insert("vega");
    if eklendi_mi {
        println!("vega eklenmis bulunmaktadir.");
    }
    println!("{:?}", veri);

    if !veri.contains("kappa") {
        println!("kappa yok bulunmaktadir.");
    }

    let eleman_sil = veri.remove("delta");
    if eleman_sil {
        println!("delta silinme evresinde");
    }
    println!("{:?}", veri);

    let _1_5 =(1..=5).collect();
    let _6_10 =(6..=10).collect();
    let _1_10 =(1..=10).collect();
    let _2_8 =(2..=8).collect();

    println!("{:?} kumesi, {:?} kumesinin alt kumesi mi: {}", _2_8, _1_10, _2_8.is_subset(&_1_10));
    println!("{:?} kumesi, {:?} kumesi ile ayrik mi: {}", _1_5, _6_10, _1_5.is_disjoint(&_6_10));
    println!("{:?} kumesi, {:?} kumesi birlesimi: {}", _2_8, _6_10, _2_8.union(&_6_10));
}
