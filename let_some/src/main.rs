#[allow(dead_code)]
#[allow(unused_variables)]

fn main() {
    let x:f64 = 2.0;
    let y:f64 = 1.0;

    let result:Option<f64> = if y!=0.0 {Some(x/y)} else {None};
    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Sifira bolme durumu")
    }
    if let Some(z) = result {
        println!("Result = {}", z);
    }
}
