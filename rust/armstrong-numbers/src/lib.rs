pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let num_str = num.to_string();
    let num_length = num_str.len();
    let result = num_str
        .split("")
        .filter(|s| !s.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .fold(0, |acc, n| acc + u32::pow(n, num_length as u32));

    result == num
}
