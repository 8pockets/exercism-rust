fn digit(n: u64) -> String {
    if n > 9 {
        panic!["Not a digit"];
    }

    let names = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"
    ];
    String::from(names[n as usize])
}

fn teen_prefix(n: u64) -> String {
    match n {
        2 => String::from("twen"),
        3 => String::from("thir"),
        4 => String::from("four"),
        5 => String::from("fif"),
        8 => String::from("eigh"),
        _ => digit(n),
    }
}

fn decimal_prefix(n: u64) -> String {
    match n {
        4 => String::from("for"),
        _ => teen_prefix(n),
    }
}

fn encode_less_1_000(n: u64) -> String {
    match n {
        _ if n < 1 || n > 999 => panic!["Only allowed for 1 to 999!"],
        _ if n > 99 => {
            let centi = digit(n / 100) + " hundred";
            let remainder = n % 100;
            if remainder > 0 {
                centi + " " + encode_less_1_000(remainder).as_str()
            } else {
                centi
            }
        }
        _ if n > 19 => {
            let deci = decimal_prefix(n / 10) + "ty";
            let remainder = n % 10;
            if remainder > 0 {
                deci + "-" + encode_less_1_000(remainder).as_str()
            } else {
                deci
            }
        }
        _ if n > 12 => teen_prefix(n - 10) + "teen",
        12 => String::from("twelve"),
        11 => String::from("eleven"),
        10 => String::from("ten"),
        _ => digit(n),
    }
}

fn _encode(n: u64) -> String {
    let names = vec![
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];
    for (i, name) in names.iter().enumerate().rev() {
        let threshold = 10u64.pow((i as u32 + 1) * 3);
        if n >= threshold {
            let encoded = _encode(n / threshold) + " " + name;
            let remainder = n % threshold;
            if remainder > 0 {
                return encoded + " " + _encode(remainder).as_str();
            } else {
                return encoded;
            }
        }
    }

    encode_less_1_000(n)
}

pub fn encode(n: u64) -> String {
    match n {
        0 => digit(n),
        _ => _encode(n),
    }
}
