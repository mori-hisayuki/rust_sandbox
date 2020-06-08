pub fn run() {
    // print to console
    println!("Hello From the print.rs files");

    // Basi Formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional Argumment
    println!("{0} is from {1} and {0} likes to {2}", "Brad", "Mass", "code");

    // Named Arguments
    println!("{name} likes to play {activity}", name = "Jojn", activity = "Baseball");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10,10,10);

    // Placeholder fo debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
  }
