#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let sicaklik:i32 = 22;
    if sicaklik > 35 {
        println!("Gercekten cok sicak!");
    } else if sicaklik < 10 {
        println!("Hava soguk");
    } else {
        println!("Hava sartlari ideal");
    }
    let gun:&str = if sicaklik > 20 {"gunesli"} else {"bulutlu"};
    println!("bugun hava {}.", gun);

    println!("hava durumu : {}", if sicaklik > 20 {"sicak"} else if sicaklik < 20 {"soguk"} else {"ideal"});

    println!("Hava durumu ozeti {}", if sicaklik > 20 {if sicaklik > 30 {"cok sicak"} else {"sicak"}} else if sicaklik < 20 {"ideal"} else {"ideal"});
}
