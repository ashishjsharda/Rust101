fn main() {
    for n in 1..101 {
        println!("{}", finbuzz(n, 15));
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    if rhs == 0 {
        return false;
    }
    lhs % rhs == 0
}

fn finbuzz(lhs: u32, rhs: u32) -> String {
    if is_divisible_by(lhs, rhs) {
        return "FizzBuzz".to_string();
    } else if is_divisible_by(lhs, 3) {
        return "Fizz".to_string();
    } else if is_divisible_by(lhs, 5) {
        return "Buzz".to_string();
    } else {
        return lhs.to_string();
    }
}
