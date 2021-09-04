pub fn run() {
  let name = "Pavel"; // let name: String = "Pavel";
  let mut age = 43;

  println!("My name is {} and i am {} years old", name, age);
  age += 1;
  println!("My name is {} and i am {} years old", name, age);

  // COnst
  const ID: i8 = 1 + 3;
  println!("ID: {}", ID);

  // Assign muntiple vars
  let (my_name, my_age) = ("Pavel", 43);
  println!("My name is {} and i am {} y.o.", my_name, my_age);
}
