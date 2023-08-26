fn main() {
    let mut x: u32 = 5;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);

    let y: bool = true;
    println!("The value of y is: {}", y);
    let y = false;
    println!("The value of y is: {}", y);

    const STRING: &str = "Hello, world!";
    println!("The value of STRING is: {}", STRING);
    println!(); // prints just a newline
    println!("hello there!");
    println!("format {} arguments", "some");
    let local_variable = "some";
    println!("format {local_variable} arguments");
}
