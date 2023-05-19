fn main() {
    let mut input = String::from("hello world");
    let mut upper = false;
    for c in input.chars().enumerate() {
        if c.1.is_alphabetic() {
            upper = !upper;
            let replacement = if upper { c.1.to_uppercase() } else { c.1.to_lowercase() };
            input.replace_range(c.0..=c.0, &replacement.collect::<String>());
        }
    }
    println!("{}", input);
}
