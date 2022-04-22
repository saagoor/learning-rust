pub fn run(){
    let name = "Sagor";
    let mut age = 24;
    println!("My name is {} and I am {}", name, age);
    age = 25;
    println!("{} is now {}", name, age);

    let (my_name, my_age) = (name, age);
    println!("My new name is {} and new age is {}", my_name, my_age);

    const ID: i16 = 12349;
    println!("ID: {}", ID);
    println!("NOTE: immutable variables and constants are seems to be the same.");
}