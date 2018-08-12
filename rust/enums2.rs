use std::f64;

enum Number {
    Real(f64),
    Complex(f64, f64),
    Quaternion(f64, f64, f64, f64),
}

trait Abs {
    fn abs(&self) -> f64;
}

impl Abs for Number {
    fn abs(&self) -> f64 {
        match self {
            Number::Real(r) if *r >= 0.0 => *r,
            Number::Real(r) if *r < 0.0 => -*r,
            Number::Real(_) => f64::NAN,
            Number::Complex(real, imag) => (real.powi(2) + imag.powi(2)).sqrt(),
            Number::Quaternion(a, b, c, d) => {
                (a.powi(2) + b.powi(2) + c.powi(2) + d.powi(2)).sqrt()
            }
        }
    }
}

fn main() {
    println!("{}", Number::Real(2.3).abs());
    println!("{}", Number::Real(-5.2).abs());
    println!("{}", Number::Complex(-2.0, 2.0).abs());
    println!("{}", Number::Quaternion(-3.0, 3.0, -3.0, -3.0).abs());
}

// NB: don't actually implement it this way!
