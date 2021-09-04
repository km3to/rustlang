pub fn run() {
    let x = 1;

    let y = 2.5;

    let z: i128 = 234324;

    // Find max size
    // println!("Max i32 is: {}", std::i32::MAX);
    // println!("Max i128 is: {}", std::i128::MAX);
    // println!("Max i128 is: {}", std::i128::MAX - 1);

    // Boolean
    let flag = false;
    let is_greater = 10 > 5;

    // Char
    let linux = '\u{1F427}';

    println!("{:?}", (x, y, z, flag, is_greater, linux));
}
