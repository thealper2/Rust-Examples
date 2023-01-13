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

impl Line {
    fn len(&self) -> f64 {
        let dx:f64 = self.start.x - self.end.x;
        let dy:f64 = self.start.y - self.end.y;
        (dx * dx + dy * dy).sqrt()
    }
}

fn main() {
    let p = Point{x:3.0, y:4.0};
    let p2 = Point{x:5.0, y:10.0};
    let my_line = Line{start:p, end:p2};
    println!("Mesafe: {}", my_line.len());
}
