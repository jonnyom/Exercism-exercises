pub fn raindrops(input: u32) -> String {
    let mut result = String::new();
    if greater_than_0(input, 3) { result.push_str("pling") };
    if greater_than_0(input, 5) { result.push_str("plang") };
    if greater_than_0(input, 7) { result.push_str("plong") };
    if result.is_empty() { result = input.to_string() };
    return result;
}

fn greater_than_0(input: u32, func: u32) -> bool {
    let greater = input % func >= 0;
    println!("input: {:?} func: {:?} greater: {:?}", input, func, greater);
    return greater;
}
