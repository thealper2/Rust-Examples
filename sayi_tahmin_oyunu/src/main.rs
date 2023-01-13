#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(unused_imports)]

use std::collections::HashMap;
use std::collections::HashSet;
use rand::Rng;
use std::io::stdin;

fn main() {
    let number:i64 = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Tahmininzi girin:");
        let mut buffer:String = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(tahmin) => {
                        if tahmin < 1 || tahmin > 100 {
                            println!("1-100 arasi bir deger girin")
                        } else if tahmin > number {
                            println!("Tahmininiz yuksek daha dusuk bir deger girin.");
                        } else if tahmin < number {
                            println!("Tahmininiz dusuk daha yuksek bir deger girin.");
                        } else {
                            println!("Tebrikler.");
                            break;
                        }
                    },
                    Err(e) => {
                        println!("Input veri okunamadi. {}. Tekrar Deneyin.", e);
                    }
                }
            },
            Err(_) => continue,
        }
    }
}
