pub fn run() {
  // Print to console
  println!("Hello from print.rs");

  // Basic formatting
  println!("{} is from {}", "Pavel", "Kyustendil");

  //Positional arguments
  println!("{0} is from {1} and {0} likes to code.", "Pavel", "Kyustendil");

  // Named arguments
  println!("{name} likes to play {activity}", name = "Pavel", activity = "WoW");

  // Placeholder args
  println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder for debug trait
  println!("{:?}", (10, true, "Hello"));

  // Basic math
  println!("10 + 10 = {}", 10 + 10 % 3);
}