#[allow(dead_code)]
#[allow(unused_variables)]



fn main() {
    let mtx:[[f32; 3]; 2] = [ [1.0, 0.0, 0.0], [0.0, 1.0, 0.0] ];
    println!("Elemanlar {:?} boyut: {}", mtx, mtx.len());

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
        }
    } 

    println!("Indis sayisina gore esit olanlar");
    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    } 
}
