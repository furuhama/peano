// Peano's national number

enum Peano {
    Zero,
    Suc,
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

fn main() {
    println!("Hello, world!");
}
