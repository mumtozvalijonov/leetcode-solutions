pub struct Solution;

impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        // Turn the String into its owned Vec<u8> buffer without reallocating
        let mut buf = s.into_bytes();

        // Two-pointer sweep over the byte array
        let mut i = 0;
        let mut j = buf.len();
        while i < j {
            // advance i to next vowel
            while i < j && !is_vowel_byte(buf[i]) {
                i += 1;
            }
            // advance j backward to previous vowel
            while i < j && !is_vowel_byte(buf[j - 1]) {
                j -= 1;
            }
            if i < j {
                buf.swap(i, j - 1);
                i += 1;
                j -= 1;
            }
        }

        // Rebuild the String from the same buffer (no new allocation)
        unsafe { String::from_utf8_unchecked(buf) }
    }
}

fn is_vowel_byte(b: u8) -> bool {
    matches!(
        b,
        b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
    )
}
