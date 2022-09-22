use flexadecimal::*;

fn main() {
    let test: usize = (Flexadecimal::from(500) + Flexadecimal::from(500)).into();
    println!("Flexadecimal: {:?}", Flexadecimal::from("10221"));
}
