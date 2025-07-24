// 1910. Remove All Occurrences of a Substring

fn main() {
    let s = String::from("axxxxyyyyb");
    let part = String::from("xy");
    let result = remove_occurrences(s, part);
    println!("{}", result);
}

fn remove_occurrences(s: String, part: String) -> String {
    let mut stack = Vec::new();

    for c in s.chars() {
        stack.push(c);

        if stack.len() >= part.len() {
            let mut found = true;
            for (i, p) in part.chars().enumerate() {
                if stack[stack.len() - part.len() + i] != p {
                    found = false;
                    break;
                }
            }

            if found {
                for _ in 0..part.len() {
                    stack.pop();
                }
            }
        }
    }

    stack.into_iter().collect()
}
