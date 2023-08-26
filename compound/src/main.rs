fn main() {
    let array : [u32; 5] = [1u32, 2, 3, 4, 5];
    println!("array: {:?}", array);
    let first_element : u32 = array[0];
    println!("first_element: {}", first_element);
    let len : usize = "some text".len();
    println!("len: {}", len);

    let slice : &[u32] = &array[1..3];
    println!("slice: {:?}", slice);
    let tuple : (u32, &str) = (1, "some text");
    println!("tuple: {:?}", tuple);
    println!("tuple.0: {}", tuple.0);
    println!("tuple.1: {}", tuple.1);

}
