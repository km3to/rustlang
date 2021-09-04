pub fn run() {
    let person: (&str, String, i8) = ("Pavel", "Gyurginchev".to_string(), 43);
    println!("{:?}", person);
    println!(
        "I am {} {} and I am {} years old.",
        person.0, person.1, person.2
    );
}
