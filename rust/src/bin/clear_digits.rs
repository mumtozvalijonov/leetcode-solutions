fn main() {
    let s = String::from("34");
    let result = clear_digits(s);
    println!("{}", result);
}

pub fn clear_digits(s: String) -> String {
    // Wir holen den internen Vektor. Da wir den String konsumieren,
    // nutzen wir `into_bytes()`. Dies liefert uns den vorhandenen Speicher.
    let mut vec = s.into_bytes();
    let mut write = 0;
    
    // Iteriere über den Vektor-Index (jedes Zeichen ist 1 Byte)
    for read in 0..vec.len() {
        let b = vec[read];
        // Überprüfe, ob das Byte eine ASCII-Ziffer ist
        if (b as char).is_ascii_digit() {
            // Falls ja, "entferne" das zuletzt geschriebene Zeichen,
            // sofern vorhanden.
            if write > 0 {
                write -= 1;
            }
        } else {
            // Ansonsten kopiere das Byte an die Stelle `write`
            vec[write] = b;
            write += 1;
        }
    }
    
    // Kürze den Vektor auf die tatsächlich benutzten Bytes
    vec.truncate(write);
    // Da wir nur Bytes verschoben bzw. entfernt haben und alle Zeichen 1 Byte groß sind,
    // bleibt die UTF-8–Gültigkeit erhalten.
    unsafe { String::from_utf8_unchecked(vec) }
}
