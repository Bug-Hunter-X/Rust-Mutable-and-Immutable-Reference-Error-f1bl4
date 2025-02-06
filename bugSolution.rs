fn main() {
    let mut x = 5;
    {
        let y = &mut x; // Mutable reference
        *y = 10;
        println!("x = {}", x); // Output: x = 10
    }

    let z = &x; // Immutable reference - safe now, because y is out of scope
    println!("x = {}", *z); // Output: x = 10
} 