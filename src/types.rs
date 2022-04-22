pub fn run() {
    let x: u8 = 120;
    let y: i16 = 12000;
    let z: f32 = 1200000000000.5555555555555555;
    println!("Types {:?}", (x, y, z));
    println!("Max of i8: {}", std::i8::MAX);
    println!("Max of i16: {}", std::i16::MAX);
    println!("Max of i64: {}", std::i64::MAX);
    println!("Max of i128: {}", std::i128::MAX);

    let character: char = 's';
    let emoji: char = '\u{1F600}';
    println!("Chars: {:?}", (character, emoji));

    let is_active: bool = true;
    let is_larger: bool = 120 < 89;
    println!("Bools {:?}", (is_active, is_larger));
}
