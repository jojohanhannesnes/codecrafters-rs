use std::env;
// Available if you need it!
// use serde_bencode

enum BencodedValue {
    String(String),
    Number(i64),
    List(Vec<BencodedValue>),
}

impl std::fmt::Debug for BencodedValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(arg0) => write!(f, "{:?}", arg0),
            Self::Number(arg0) => write!(f, "{:?}", arg0),
            BencodedValue::List(arg0) => write!(f, "{:?}", arg0),
        }
    }
}
impl Into<BencodedValue> for i64 {
    fn into(self) -> BencodedValue {
        BencodedValue::Number(self)
    }
}

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> BencodedValue {
    match encoded_value {
        // integer
        int_bencode if int_bencode.starts_with('i') => {
            let int_bencode = int_bencode
                .get(1..int_bencode.len() - 1)
                .unwrap_or_else(|| panic!("Error slicing the integer"))
                .parse::<i64>()
                .unwrap_or_else(|e| panic!("Error parsing integer {}", e));
            int_bencode.into()
        }
        // vector
        mut x if x.starts_with('l') => {
            let mut lists: Vec<&str> = Vec::new();
            x = x.strip_prefix('l').unwrap();
            while x.len() != 1 {
                // reached last e
                let delim: usize;
                match x.chars().next().unwrap() {
                    'i' => {
                        delim = x.find('e').unwrap();
                        let digits = x.get(0..=delim).unwrap();
                        x = &x[delim + 1..];
                        lists.push(digits);
                    }
                    y if y.is_ascii_digit() => {
                        delim = x.find(':').unwrap();
                        let encode_length = x.get(0..delim).unwrap().parse::<usize>().unwrap();
                        let encode_value = x.get(0..=delim + encode_length).unwrap();
                        x = &x[delim + 1 + encode_length..];
                        lists.push(encode_value);
                    }
                    _ => panic!("doesnt understand the list"),
                }
            }
            let x: Vec<BencodedValue> = lists.into_iter().map(decode_bencoded_value).collect();
            BencodedValue::List(x)
        }
        // string
        x if x.contains(':') => {
            if let Some((_, right)) = x.split_once(':') {
                BencodedValue::String(right.to_string())
            } else {
                panic!("[string]Unhandled encoded value: {}", x)
            }
        }
        _ => panic!("unknown value"),
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{:?}", decoded_value);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
