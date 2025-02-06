fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &x; // z is an immutable reference to x

    *y = 10; // Modify x through y
    println!("x = {}", x); // Output: x = 10

    // The following line causes a compile-time error because z is an immutable reference,
    // and we're trying to modify x through a mutable reference y while z exists.
    // *y = 20;
}