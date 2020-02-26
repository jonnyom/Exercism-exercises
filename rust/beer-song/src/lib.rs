pub fn verse(n: u32) -> String {
    sing_verse(n)
}

pub fn sing(start: u32, end: u32) -> String {
    let verses: Vec<String> = if start > end {
        (end..start + 1).rev().map(|nth| verse(nth as u32)).collect()
    } else {
        (start..end).map(|nth| verse(nth as u32)).collect()
    };
    verses.join("\n")
}

fn sing_verse(n: u32) -> String {
    let bottles = Bottles(n);
    let mut verse = String::new();
    let top_line = format!(
        "{} of beer on the wall, {} of beer.\n",
        bottles.bottle_count().first_letter_to_uppper_case(),
        bottles.bottle_count()
    );
    verse.push_str(&top_line);
    match n {
        0 => verse.push_str("Go to the store and buy some more, 99 bottles of beer on the wall.\n"),
        _ => verse.push_str(&(format!("{}", bottom_line(n)))),
    };
    verse
}

fn bottom_line(n: u32) -> String {
    let less_bottles = Bottles(n - 1);
    let how_many_to_take = match n {
        1 => "it",
        _ => "one",
    };
    let bottom_line = format!(
        "Take {} down and pass it around, {} of beer on the wall.\n",
        how_many_to_take,
        less_bottles.bottle_count()
    );
    String::from(&bottom_line)
}

struct Bottles(u32);

impl Bottles {
    fn bottle_count(&self) -> String {
        match self.0 {
            0 => String::from("no more bottles"),
            1 => String::from("1 bottle"),
            _ => String::from(&(format!("{} bottles", self.0))),
        }
    }
}

pub trait FirstLetterToUppperCase {
    fn first_letter_to_uppper_case(self) -> String;
}

impl FirstLetterToUppperCase for String {
    fn first_letter_to_uppper_case(self) -> String {
        let mut c = self.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
