pub fn encode(arg: &[u8], strict: bool, all: bool) -> String {
    let mut encoded = String::new();
    for c in arg {
        if all {
            encoded.push_str(&format!("%{:02X}", c))
        } else {
            match c {
                b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                    if strict && b"-_.~".contains(&c) {
                        encoded.push_str(&format!("%{:02X}", c))
                    } else {
                        encoded.push(*c as char)
                    }
                }
                _ => encoded.push_str(&format!("%{:02X}", c)),
            }
        }
    }
    encoded
}

