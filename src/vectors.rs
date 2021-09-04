pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];
    numbers.push(5);
    let el = numbers.pop();

    println!("Popped value: {:?}", el);
    println!("{:?}", numbers);

    for xx in numbers {
        println!("{}", xx)
    }
}
