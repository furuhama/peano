// Peano's national number

#[derive(Debug)]
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

    fn suc(&'a self) -> Peano<'a> {
        Peano::Suc(self)
    }
}

fn zero<'a>() -> Peano<'a> {
    Peano::Zero
}

fn _suc<'a>(n: &'a Peano) -> Peano<'a> {
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

#[test]
fn test_impl_suc() {
    assert!(match zero().suc() {
        Peano::Suc(_) => true,
        _ => false,
    });
}

fn main() {
    println!("Peano::Zero -> {:?}", zero());
    println!("zero().to_i() -> {:?}", zero().to_i());
    println!("zero().suc() -> {:?}", zero().suc());
    println!("zero().suc().to_i() -> {:?}", zero().suc().to_i());
    println!("zero().suc().suc() -> {:?}", zero().suc().suc());
    println!("zero().suc().suc().to_i() -> {:?}", zero().suc().suc().to_i());
}
