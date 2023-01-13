#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let s:&'static str = "merhaba"; //static utf-8

    for c in s.chars().rev() {
        println!("{}", c);
    }
    if let Some(ilk_karakter) = s.chars().nth(0) {
        println!("Ilk karakter: {}", ilk_karakter);
    }

    //heap
    let mut karakterler = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        karakterler.push(a as char);
        karakterler.push_str(",");
        a += 1;
    }
    println!("{}", karakterler);
    let u:&str = &karakterler;

    let mut abc = "merhaba dunya".to_string();
    abc.remove(0);
    let sss = abc.replace("a", "c");
    println!("{}", sss);
    abc.push_str("!!!");
    println!("{}", abc);
    println!("{}, {}", abc, abc.replace("erhaba", "hoscakal"));
}
