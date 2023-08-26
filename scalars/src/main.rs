fn main() {
    let a: u8 = 55;
    let b: u8 = 1;
    let x: f64 = 3.0;
    let y: f32 = 2.0;

    println!("x: {}, y: {}", x, y);

    let sum: u8 = a + b;
    let diff: u8 = a - b;
    let prod: u8 = a * b;
    let quot: u8 = a / b;
    let rem: u8 = a % b;

    println!(
        "sum: {}, diff: {}, prod: {}, quot: {}, rem: {}",
        sum, diff, prod, quot, rem
    );

    let b: bool = true;
    let c: char = 'x';
    let emoji: char = '🤣';

    println!("b: {}, c: {}, emoji: {}", b, c, emoji);

    let d: char = 'é';
    println!("d: {}", d);
    let _k = 'ℤ';
    println!("k: {}", _k);
}
