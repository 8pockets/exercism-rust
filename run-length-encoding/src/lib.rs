pub fn encode(s: &str) -> String {
    let mut output = String::new();
    let mut i = 0;
    let input = s.as_bytes();

    while i < input.len() {
        let mut count = 1;
        let ch = input[i] as char;
        i += 1;
        while i < input.len() && input[i] == ch as u8 {
            count += 1;
            i += 1;
        }
        if count == 1 {
            output.push(ch);
        } else {
            output.push_str(count.to_string().as_str());
            output.push(ch);
        }
    }
    output
}

pub fn decode(s: &str) -> String {
    let input = s.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < input.len() {
        let mut count = 0;
        while (input[i] as char).is_numeric() {
            count *= 10;
            count += input[i] - b'0';
            i += 1;
        }
        if count == 0 {
            count = 1;
        }
        for _ in 0..count {
            out.push(input[i] as char);
        }
        i += 1;
    }
    out
}
