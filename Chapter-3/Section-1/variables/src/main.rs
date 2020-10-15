fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing ex.
    let y = x + 1;
    let y = x + 2;

    println!("y = x + 2: {}", y == x + 2);
}
