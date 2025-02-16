fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y += 1; // Modify x through y. This is allowed.
    // *z += 1; // This is not allowed. It would cause a compile-time error.
    println!("x = {}", x); // Prints: x = 6
}