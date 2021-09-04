pub fn run() {
    let hello = "Hello";
    let hello_len = hello.len();
    let mut bye = String::from("Bye");
    bye.push(' ');
    bye.push_str("W");
    let test = bye.push_str("orld!");
    let bye_len = bye.len();

    println!("{}'s len = {}", hello, hello_len);
    println!("{}'s len = {}", bye, bye_len);
    println!("{:?}", test);

    println!("Capacity: {}", bye.capacity());
    println!("Is empty: {}", bye.is_empty());

    let contains = bye.contains("orl");
    println!("Contains 'orl' {}", contains);
    bye = bye.replace("orl", "ORL");

    println!("{}", bye);

    // Loop through string by whitespace
    for token in bye.split_ascii_whitespace() {
        println!("{}", token)
    }

    for token in bye.split(" ") {
        println!("{}", token)
    }

    assert_eq!(3, 3);
    assert_ne!(3, 4);
    println!("That should not be printed!");
}
