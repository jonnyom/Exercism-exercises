pub fn is_armstrong_number(num: u32) -> bool {
    let numbers = number_to_vec(num);
    let length = numbers.len() as u32;
    numbers.iter().map(|i| i.pow(length)).sum::<u32>() == num
}

fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}
