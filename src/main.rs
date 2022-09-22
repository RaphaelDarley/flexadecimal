use flexadecimal::*;

fn main() {
    let test: usize = (Flexadecimal::from(500) + Flexadecimal::from("321")).into();
    println!("Flexadecimal: {:?}", test);
    assert_eq!(test, 523);
    assert_eq!(42usize, Flexadecimal::from(42).into());
}
