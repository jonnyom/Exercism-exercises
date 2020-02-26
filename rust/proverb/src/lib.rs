pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() { return String::new(); }
    let mut proverb: Vec<String> = (1..list.len())
        .map(|i| {
            String::from(&(format!("For want of a {} the {} was lost.", list[i - 1], list[i])))
        })
        .collect();
    proverb.push(String::from(&format!(
        "And all for the want of a {}.",
        list[0]
    )));
    proverb.join("\n")
}
