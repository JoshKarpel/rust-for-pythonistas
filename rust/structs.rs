use std::fmt;

struct Position {
    x: f64,
    y: f64,
}

impl Position {
    // equivalent of a method
    fn distance_to_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


impl fmt::Display for Position {
    // equivalent of a magic method (__str__, in this case)
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x={},y={})", self.x, self.y)
    }
}


fn main() {
    let p = Position { x: 3.0, y: 4.0 };
    println!("{}", p);
    println!("{}", p.distance_to_origin());
}
