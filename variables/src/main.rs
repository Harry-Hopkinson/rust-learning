fn main() {
    // a mutable variable is one that can be changed
    let mut x = 5;
    // not a mutable variables - so a constant
    let y = 5;
    println!("Hello, world!");
    x = 6;
    // y = 6; // this will give an error
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);

    const PI: f32 = 3.141592;
    println!("The value of PI is: {}", PI);
}
