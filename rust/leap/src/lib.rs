pub fn is_leap_year(year: u64) -> bool {
    let divisible_by_four = year % 4 == 0;
    let divisible_by_100 = year % 100 == 0;
    let divisible_by_400 = year % 400 == 0;

    return divisible_by_four && (!divisible_by_100 || divisible_by_400);
}
