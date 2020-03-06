pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<String> = Vec::new();
    for elem in string.chars() {
        let elem = elem.to_string();
        if elem == "{" || elem == "[" || elem == "(" {
            stack.push(elem.clone());
        };
        if elem == "}" || elem == "]" || elem == ")" {
            if stack.is_empty() || !is_matching_pair(&stack.pop().unwrap(), &elem.to_string()) {
                return false;
            }
        }
    }
    stack.is_empty()
}

fn is_matching_pair(c1: &str, c2: &str) -> bool {
    match c1 {
        x if x == "{" && c2 == "}" => true,
        x if x == "(" && c2 == ")" => true,
        x if x == "[" && c2 == "]" => true,
        _ => false,
    }
}
