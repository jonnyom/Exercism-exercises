pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb: Vec<String> = (1..list.len()).step_by(2)
        .map(|i| {
            String::from(&(format!("For want of a {} the {} was lost.", list[i - 1], list[i])))
        })
        .collect();
    proverb.push(String::from(&format!(
        "And all for the want of a {}.",
        list[0]
    )));
    println!("{:?}", proverb);
    proverb.join("\n")
}
