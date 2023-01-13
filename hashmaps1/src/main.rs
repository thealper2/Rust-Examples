#[allow(dead_code)]
#[allow(unused_variables)]

use std::collections::HashMap;

fn main() {
   let mut sekiller = HashMap::new();

   let ucgen = String::from("ucgen");
   sekiller.insert(ucgen, 3);
   sekiller.insert("kare".into(), 4);

   println!("bir karenin {} kenari vardir", sekiller["kare".into()]);

   for (k, v) in &sekiller {
    println!("{} : {}", k, v);
   }
   println!("-----------------");

   sekiller.insert("kare".into(), 66);
   println!("{:?}", sekiller);

   sekiller.entry("daire".into()).or_insert(444);
   {
    let suan = sekiller.entry("yamuk".into()).or_insert(1234);
    *suan = 0;
   }
   println!("{:?}", sekiller);
}  
