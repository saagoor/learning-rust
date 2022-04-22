pub fn run() {
    // Print line
    println!("Hello there, how you doin in main file?");

    // Print line with string interpollation
    println!("Why is this printing at the top? {}", "Brad");

    // Print line with named argument
    println!(
        "{what} named variables by {whoo} for {why}",
        whoo = "Sagor",
        what = "Printing",
        why = "Learning"
    );

    // Print line with data types
    println!(
        "Binary {:b} Hex {:X} Octal {:0} Decimal {:E}",
        10, 10, 10, 1000
    );

    // Print line with placeholder for debug trait
    println!("{:?}", (12, true, "This is awesome"));

    // Print line with basic math
    println!("10 + 10 = {}", 10 + 10);
}
