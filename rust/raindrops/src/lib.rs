pub fn raindrops(input: u32) -> String {
    let mut result = String::new();
    if factor_of(input, 3) { result.push_str("Pling") };
    if factor_of(input, 5) { result.push_str("Plang") };
    if factor_of(input, 7) { result.push_str("Plong") };
    if result.is_empty() { result = input.to_string() };
    return result;
}

fn factor_of(input: u32, func: u32) -> bool {
    return input % func == 0;
}
