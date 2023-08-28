// classic struct definition
struct Person {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    age: u8,
}


// tuple struct definition
struct Point2D(u32, u32);



fn main() {
    // classic struct instiantiation
    println!("classic struct instiantiation");
    let user1 = Person {
        username: String::from("Daly"),
        email: String::from("daly@gmail.com"),
        active: true,
        sign_in_count: 1,
        age: 20,
    };
    println!("user1 name is {}", user1.username);
    println!("user1 email is {}", user1.email);
    println!("user1 active is {}", user1.active);
    println!("user1 sign_in_count is {}", user1.sign_in_count);
    println!("user1 age is {}", user1.age);

    // tuple struct instiantiation

    println!("tuple struct instiantiation");

    let origin = Point2D(100, 200);
    println!("origin x is {:?} and {:?}", origin.0,origin.1);

    let Point2D(x,y)= origin;
    println!("origin x is {:?} and {:?}", x,y);

}
