#[allow(dead_code)]
#[allow(unused_variables)]

struct Point<T, V> {
    x:T,
    y:V
}

struct Line<T, V> {
    start:Point<T,V>,
    end:Point<T,V>
}

fn main() {
    let a = Point{x:4, y:7};
    let b = Point{x:4.4, y:7.2};
    let c = Point{x:4.4, y:7};
    let cizgi = Line{start:Point{x:2, y:2.5}, end:Point{x:4, y:5.6}};
    println!("c = ({}, {})", c.x, c.y);
    println!("a = ({}, {})", a.x, a.y);
    println!("b = ({}, {})", b.x, b.y);
}
