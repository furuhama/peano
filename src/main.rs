// Peano's national number

enum Peano<'a> {
    Zero,
    Suc(&'a Peano<'a>),
}

impl <'a> Peano<'a> {
    fn to_i(&self) -> u32 {
        match *self {
            Peano::Zero => 0,
            _ => 1,
        }
    }
}

fn zero<'a>() -> Peano<'a> {
    Peano::Zero
}

fn suc<'a>(n: &'a Peano) -> Peano<'a> {
    Peano::Suc(n)
}

#[test]
fn test_zero() {
    assert!(match zero() {
        Peano::Zero => true,
        _ => false,
    });
}

#[test]
fn test_to_i() {
    assert!(zero().to_i() == 0);
}

#[test]
fn test_suc() {
    assert!(match suc(&zero()) {
        Peano::Suc(_) => true,
        _ => false,
    });
}

fn main() {
    println!("Hello, world!");
}
