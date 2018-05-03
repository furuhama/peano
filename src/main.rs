// Peano's national number

enum Peano {
    Zero,
    Suc,
}

impl Peano {
    fn to_i(&self) -> u32 {
        match *self {
            Peano::Zero => 0,
            _ => 1
        }
    }
}

fn zero() -> Peano {
    Peano::Zero
}

#[test]
fn test_zero() {
    assert!(match zero() {
        Peano::Zero => true,
        _ => false
    });
}

#[test]
fn test_to_i() {
    assert!(zero().to_i() == 0);
}

fn main() {
    println!("Hello, world!");
}
