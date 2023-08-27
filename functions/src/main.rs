fn main() {
    println!("FUNCTIONS");
    print_excited("dvlynsk".to_string());

    last_char(String::from("Dvlynsk"));
}

fn exclaim(input: String) -> String {
    let mut output = input.to_uppercase();
    output.push_str("!");
    output
}

fn print_excited(input: String) -> () {
    let output = exclaim(input);
    println!("{}", output);
}

fn last_char(input: String) -> char {
    if input.is_empty() {
        print!("Empty string");
        return 'z';
    }
    print!("{}", input.chars().next_back().unwrap());
    input.chars().next_back().unwrap()
}
