pub fn is_palindrome(number: u128) -> bool {
    let reversed = number.to_string().chars().rev().collect::<String>().parse::<u128>().unwrap();

    return number == reversed;
}