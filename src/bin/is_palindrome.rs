fn main() {
    let x = 202;
    let result = is_palindrome(x);
    println!("{}", result);

}

fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    } else if x < 10 {
        return true;
    } else if x % 10 == 0 {
        return false;
    } else if x < 100 {
        return x % 11 == 0;
    } else if x < 1000 {
        return x % 10 == x / 100;
    } else if x < 10_000 {
        return x % 10 == x / 1000 && x % 100 / 10 == x / 100 % 10;
    } else if x < 100_000 {
        return x % 10 == x / 10000 && x % 100 / 10 == x / 1000 % 10;
    } else if x < 1_000_000 {
        return x % 10 == x / 100_000 && x % 100 / 10 == x / 10_000 % 10 && x % 1000 / 100 == x / 1000 % 10;
    } else if x < 10_000_000 {
        return x % 10 == x / 1_000_000 && x % 100 / 10 == x / 100_000 % 10 && x % 1000 / 100 == x / 10_000 % 10;
    } else if x < 100_000_000 {
        return x % 10 == x / 10_000_000 && x % 100 / 10 == x / 1_000_000 % 10 && x % 1000 / 100 == x / 100_000 % 10;
    } else if x < 1_000_000_000 {
        return x % 10 == x / 100_000_000 && x % 100 / 10 == x / 10_000_000 % 10 && x % 1000 / 100 == x / 1_000_000 % 10 && x % 10_000 / 1000 == x / 100_000 % 10;
    }
    return x % 10 == x / 1_000_000_000 && x % 100 / 10 == x / 100_000_000 % 10 && x % 1000 / 100 == x / 10_000_000 % 10 && x % 10_000 / 1000 == x / 1_000_000 % 10 && x % 100_000 / 10_000 == x / 100_000 % 10;
}

