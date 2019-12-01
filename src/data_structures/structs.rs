struct Point {
    x: f64,
    y: f64,
}

struct Line {
    start: Point,
    end: Point
}

pub fn structs() {
    let p = Point {
        x: 3.0,
        y: 4.0
    };

    let p2 = Point {
        x: 5.0,
        y: 10.0
    };
    
    let my_line = Line {start: p, end: p2};

}
