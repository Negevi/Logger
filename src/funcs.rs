fn get_input(q: &str) -> String {
    let mut input = String::new();
    println!("{q}");
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}
