#[allow(dead_code)]
#[allow(unused_variables)]

struct Point {
    x:f64,
    y:f64
}

struct Line {
    start:Point,
    end:Point
}

fn main() {
    let p = Point{x:4.0, y:3.0};
    println!("Point p is ({}, {})", p.x, p.y);
    let p2 = Point{x:5.0, y:10.0};
    let my_line = Line{start:p, end:p2};
    println!("My line is on ({}, {}) - ({}, {}) points", my_line.start.x, my_line.start.y, my_line.end.x, my_line.end.y);
}
