fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    println!("{}", lps_kmp(line.trim().to_string()));
}

fn lps_kmp(pattern: String) -> u8 {
    let pattern_size = pattern.len();
    if pattern_size <= 1 {
        return 0;
    }
    let pattern: Vec<char> = pattern.chars().collect();
    let mut lps: Vec<u8> = vec![0; pattern_size];
    let (mut i, mut length) = (1, 0);

    while i < pattern_size {
        if pattern[i] == pattern[length] {
            length += 1;
            lps[i] = length as u8;
            i += 1;
        } else {
            if length != 0 {
                length = lps[length - 1] as usize;
            } else {
                lps[i] = 0;
                i += 1;
            }
        }
    }

    println!("{:?}", lps);
    lps[pattern_size - 1]
}
