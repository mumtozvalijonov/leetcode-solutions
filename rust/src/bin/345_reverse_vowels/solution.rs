pub struct Solution;

impl Solution {
    pub fn reverse_vowels(mut s: String) -> String {
        let bytes = unsafe { s.as_bytes_mut() };
        let mut vowels = bytes.iter_mut().filter(|c| is_vowel_byte(**c));

        while let (Some(left), Some(right)) = (vowels.next(), vowels.next_back()) {
            std::mem::swap(left, right);
        }

        s
    }
}

fn is_vowel_byte(b: u8) -> bool {
    matches!(
        b,
        b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
    )
}
