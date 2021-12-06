use std::{cmp::Ordering, fmt::{Display, Error, Formatter}, ops::{Add, Div, Mul, Neg, Rem, Sub}};

// use macros from crate
#[macro_use]
extern crate lazy_static;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Nat {
    One,
    Succ(Box<Nat>),
}

impl Display for Nat {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_i32())        
    }
}

impl Add for Nat {
    type Output = Nat;

    fn add(self, other: Nat) -> Nat {
        use Nat::*;
        match self {
            One => Succ(Box::new(other)),
            Succ(value) => Succ(Box::new(value.add(other))),
        }
    }
}

impl Sub for Nat {
    type Output = Nat;

    fn sub(self, other: Nat) -> Nat {
        use Nat::*;
        match other {
            One => {
                if let Succ(predecessor) = self {
                    *predecessor
                } else {
                    panic!()
                }
            }
            Succ(value) => {
                if let Succ(predecessor) = self {
                    predecessor.sub(*value)
                } else {
                    panic!()
                }
            }
        }
    }
}

impl Mul for Nat {
    type Output = Nat;

    fn mul(self, other: Nat) -> Nat {
        use Nat::*;
        match self {
            One => other,
            Succ(value) => value.mul(other.clone()).add(other),
        }
    }
}

impl PartialOrd for Nat {
    fn partial_cmp(&self, other: &Nat) -> Option<Ordering> {
        use Nat::*;
        match (self, other) {
            (One, One) => Some(Ordering::Equal),
            (One, _) => Some(Ordering::Less),
            (_, One) => Some(Ordering::Greater),
            (Succ(value), Succ(other)) => value.partial_cmp(other),
        }
    }
}

impl Ord for Nat {
    fn cmp(&self, other: &Nat) -> Ordering {
        use Nat::*;
        match (self, other) {
            (One, One) => Ordering::Equal,
            (One, _) => Ordering::Less,
            (_, One) => Ordering::Greater,
            (Succ(value), Succ(other)) => value.cmp(other),
        }
    }
}

impl Div for Nat {
    type Output = Nat;

    fn div(self, other: Nat) -> Nat {
        use Nat::*;

        let (mut a, b) = (self, other);
        let mut times = One;
        while a > b {
            a = a - b.clone();
            times = times + One;
        }

        times
    }
}

impl Nat {
    fn from_int(value: i32) -> Nat {
        use Nat::*;
        assert!(value >= 1);
        match value {
            1 => One,
            _ => Succ(Box::new(Nat::from_int(value - 1))),
        }
    }

    fn as_i32(&self) -> i32 {
        use Nat::*;
        match self {
            One => 1,
            Succ(value) => 1 + value.as_i32(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Int {
    Zero,
    Pos(Nat),
    Neg(Nat),
}

impl Add for Int {
    type Output = Int;

    fn add(self, other: Int) -> Int {
        use Int::*;
        match (self, other) {
            (Zero, other) => other,
            (s, Zero) => s,
            (Pos(a), Pos(b)) => Pos(a + b),
            (Neg(a), Neg(b)) => Neg(a + b),
            (Pos(a), Neg(b)) => {
                match a.cmp(&b) {
                    Ordering::Greater => Pos(a - b),
                    Ordering::Less => Neg(b - a),
                    Ordering::Equal => Zero,
                }
            }
            (Neg(a), Pos(b)) => {
                match a.cmp(&b) {
                    Ordering::Greater => Neg(a - b),
                    Ordering::Less => Pos(b - a),
                    Ordering::Equal => Zero,
                }
            }
        }
    }
}

impl Neg for Int {
    type Output = Int;

    fn neg(self) -> Int {
        use Int::*;
        match self {
            Zero => Zero,
            Pos(value) => Neg(value),
            Neg(value) => Pos(value),
        }
    }
}

impl Sub for Int {
    type Output = Int;

    fn sub(self, other: Int) -> Int {
        self + (-other)
    }
}

impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Int) -> Option<Ordering> {
        use Int::*;
        match (self, other) {
            (Zero, Zero) => Some(Ordering::Equal),
            (Zero, Pos(_)) => Some(Ordering::Less),
            (Pos(_), Zero) => Some(Ordering::Greater),
            (Zero, Neg(_)) => Some(Ordering::Greater),
            (Neg(_), Zero) => Some(Ordering::Less),
            (Pos(a), Pos(b)) => a.partial_cmp(b),
            (Neg(a), Neg(b)) => b.partial_cmp(a),
            (Pos(_), Neg(_)) => Some(Ordering::Greater),
            (Neg(_), Pos(_)) => Some(Ordering::Less),
        }
    }
}

impl Ord for Int {
    fn cmp(&self, other: &Int) -> Ordering {
        use Int::*;
        match (self, other) {
            (Zero, Zero) => Ordering::Equal,
            (Zero, Pos(_)) => Ordering::Less,
            (Pos(_), Zero) => Ordering::Greater,
            (Zero, Neg(_)) => Ordering::Greater,
            (Neg(_), Zero) => Ordering::Less,
            (Pos(a), Pos(b)) => a.cmp(b),
            (Neg(a), Neg(b)) => b.cmp(a),
            (Pos(_), Neg(_)) => Ordering::Greater,
            (Neg(_), Pos(_)) => Ordering::Less,
        }
    }
}

impl Div for Int {
    type Output = Int;

    fn div(self, other: Int) -> Int {
        use Int::*;
        
        match (self, other) {
            (Zero, _) => Zero,
            (_, Zero) => panic!("division by zero!"),
            (Pos(a), Pos(b)) => Pos(a / b),
            (Neg(a), Neg(b)) => Pos(a / b),
            (Pos(a), Neg(b)) => Neg(a / b),
            (Neg(a), Pos(b)) => Neg(a / b),
        }
    }
}

impl Rem for Int {
    type Output = Int;

    fn rem(self, other: Int) -> Int {
        use Int::*;

        let mut a = self;
        let b = other;

        if a == Zero {
            return b;
        }

        if b == Zero {
            return a;
        }

        while a >= b {
            a = a - b.clone();
        }

        a
    }
}

impl Mul for Int {
    type Output = Int;

    fn mul(self, other: Int) -> Int {
        use Int::*;
        
        match (self, other) {
            (Zero, _) => Zero,
            (_, Zero) => Zero,
            (Pos(a), Pos(b)) => Pos(a * b),
            (Neg(a), Neg(b)) => Pos(a * b),
            (Pos(a), Neg(b)) => Neg(a * b),
            (Neg(a), Pos(b)) => Neg(a * b),
        }
    }
}

impl Display for Int {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}", self.as_i32())
    }
}

impl Int {
    #[allow(dead_code)]
    fn abs(self) -> Int {
        use Int::*;
        match self {
            Zero => Zero,
            Pos(value) => Pos(value),
            Neg(value) => Pos(value),
        }
    }

    fn from_int(value: i32) -> Int {
        use Int::*;
        match value.cmp(&0) {
            Ordering::Less => Neg(Nat::from_int(-value)),
            Ordering::Equal => Zero,
            Ordering::Greater => Pos(Nat::from_int(value)),
        }
    }

    fn as_i32(&self) -> i32 {
        use Int::*;
        match self {
            Zero => 0,
            Pos(value) => value.as_i32(),
            Neg(value) => -value.as_i32(),
        }
    }
}

fn gcd(mut x: Int, mut y: Int) -> Int {
    while y != Int::Zero {
        let temp = y.clone();
        y = x % y;
        x = temp;
    }

    x
}

impl Neg for Frac {
    type Output = Frac;

    fn neg(self) -> Frac {
        Frac {
            num: -self.num,
            den: self.den,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Frac {
    num: Int,
    den: Int,
}

impl Frac {
    fn new(num: Int, den: Int) -> Frac {
        Frac::abnormalised(num, den).normalise()
    }

    fn from_int(num: Int) -> Frac {
        Frac::new(num, Int::Pos(Nat::One))
    }

    fn abs(self) -> Frac {
        Frac {
            num: self.num.abs(),
            den: self.den.abs(),
        }
    }

    fn abnormalised(num: Int, den: Int) -> Frac {
        Frac { num, den }
    }

    fn normalise(self) -> Frac {
        use Int::*;
        use Nat::*;

        let out = match (self.num, self.den) {
            (Neg(num), Neg(den)) => Frac::abnormalised(Pos(num), Pos(den)),
            (Neg(num), Pos(den)) => Frac::abnormalised(Neg(num), Pos(den)),
            (Pos(num), Neg(den)) => Frac::abnormalised(Neg(num), Pos(den)),
            (Pos(num), Pos(den)) => Frac::abnormalised(Pos(num), Pos(den)),
            (Zero, Zero) => Frac::abnormalised(Zero, Zero),
            (Zero, _) => Frac::abnormalised(Zero, Pos(One)),
            (x, Zero) => Frac::abnormalised(x, Zero),
        };
        let gcd = gcd(out.num.clone().abs(), out.den.clone().abs());
        Frac::abnormalised(out.num / gcd.clone(), out.den / gcd)
    }
}

impl Mul for Frac {
    type Output = Frac;

    fn mul(self, other: Frac) -> Frac {
        Frac::new(self.num * other.num, self.den * other.den)
    }
}

impl Div for Frac {
    type Output = Frac;

    fn div(self, other: Frac) -> Frac {
        Frac::new(self.num * other.den, self.den * other.num)
    }
}

impl Rem for Frac {
    type Output = Frac;

    fn rem(mut self, other: Frac) -> Frac {
        assert!(self.clone().abs() == self);
        while self >= other {
            self = self - other.clone();
        }
        self
    }
}

impl Display for Frac {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        use Int::*;
        use Nat::*;

        if self.den == Pos(One) {
            write!(f, "{}", self.num)
        } else {
            write!(f, "{}/{}", self.num, self.den)
        }
    }
}

impl Add for Frac {
    type Output = Frac;

    fn add(self, other: Frac) -> Frac {
        let (a, b) = (self.num, self.den);
        let (c, d) = (other.num, other.den);
        Frac::new(a * d.clone() + b.clone() * c, b * d)
    }
}

impl Sub for Frac {
    type Output = Frac;

    fn sub(self, other: Frac) -> Frac {
        let (a, b) = (self.num, self.den);
        let (c, d) = (other.num, other.den);
        Frac::new(a * d.clone() - b.clone() * c, b * d)
    }
}

impl PartialOrd for Frac {
    fn partial_cmp(&self, other: &Frac) -> Option<Ordering> {
        let result = self.clone() - other.clone();

        Some(result.num.cmp(&Int::Zero))
    }
}

impl Ord for Frac {
    fn cmp(&self, other: &Frac) -> Ordering {
        let result = self.clone() - other.clone();

        result.num.cmp(&Int::Zero)
    }
}

lazy_static! {
static ref PI: Frac = {
    Frac::new(Int::from_int(355), Int::from_int(113))
};
static ref TAU: Frac = {
    Frac::new(Int::from_int(355 * 2), Int::from_int(113))
};
}

fn sin(x: Frac) -> Frac {
    let x = x % TAU.clone();
    let fac3 = Frac::from_int(Int::from_int(6));
    let fac5 = Frac::from_int(Int::from_int(120));
    let fac7 = Frac::from_int(Int::from_int(5040));
    x.clone() - (x .clone()* x.clone() * x.clone() / fac3) + (x.clone() * x.clone() * x.clone() * x.clone() * x.clone() / fac5) - (x.clone() * x.clone() * x.clone() * x.clone() * x.clone() * x.clone() * x / fac7)
}

fn cos(x: Frac) -> Frac {
    let x = x % TAU.clone();
    let fac2 = Frac::from_int(Int::from_int(2));
    let fac4 = Frac::from_int(Int::from_int(24));
    let fac6 = Frac::from_int(Int::from_int(720));
    let one = Frac::from_int(Int::from_int(1));
    one - (x.clone() * x.clone() / fac2) + (x.clone() * x.clone() * x.clone() * x.clone() / fac4) - (x.clone() * x.clone() * x.clone() * x.clone() * x.clone() * x / fac6)
}

fn tan(x: Frac) -> Frac {
    sin(x.clone()) / cos(x)
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Complex {
    real: Frac,
    imag: Frac,
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        // foil
        let f = self.real.clone() * other.real.clone();
        let o = self.real * other.imag.clone();
        let i = self.imag.clone() * other.real;
        let l = self.imag * other.imag;
        Complex {
            real: f - l,
            imag: o + i,
        }
    }
}

impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let conjugate = Complex {
            real: other.real.clone(),
            imag: -other.imag.clone(),
        };

        let top = self.mul(conjugate);
        let bottom = other.real.clone() * other.real + other.imag.clone() * other.imag;

        Complex {
            real: top.real / bottom.clone(),
            imag: top.imag / bottom,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let isign = if self.imag < Frac::new(Int::Zero, Int::Pos(Nat::One)) {
            "-"
        } else {
            "+"
        };
        write!(f, "{} {} {}i", self.real, isign, self.imag.clone().abs())
    }
}

impl Complex {
    fn new(real: Frac, imag: Frac) -> Complex {
        Complex { real, imag }
    }

    fn from_ints(real: Int, imag: Int) -> Complex {
        let f1 = Frac::from_int(real);
        let f2 = Frac::from_int(imag);
        Complex::new(f1, f2)
    }

    fn as_polar(&self) -> String {
        let magnitude = self.real.clone() * self.real.clone() + self.imag.clone() * self.imag.clone();
        let imag_over_real = self.imag.clone() / self.real.clone();
        let angle = atan(imag_over_real);
    }
}

fn main() {
    let c1 = Complex::from_ints(Int::from_int(7), Int::from_int(-4));
    let c2 = Complex::from_ints(Int::from_int(3), Int::from_int(2));

    println!("({}) * ({}) = {}", c1, c2, c1.clone() * c2.clone());
    println!("({}) / ({}) = {}", c1, c2, c1.clone() / c2.clone());
    println!();
    println!("{} internal representation: {:?}", c1, c1);
    println!("{} internal representation: {:?}", c2, c2);
}
