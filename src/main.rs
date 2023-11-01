use std::env;

// Available if you need it!
// use serde_bencode

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> String {
    match encoded_value {
        // integer
        value if value.starts_with('i') && value.ends_with('e') => {
            let x = value
                .get(1..value.len() - 1)
                .unwrap_or_else(|| panic!("Error slicing the integer"))
                .parse::<i32>()
                .unwrap_or_else(|e| panic!("Error parsing integer {}", e));
            x.to_string()
        }
        // string
        val if val.contains(':') => {
            if let Some((_, right)) = val.split_once(':') {
                right.to_string()
            } else {
                panic!("[string]Unhandled encoded value: {}", val)
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
        println!("{}", decoded_value);
    } else {
        println!("unknown command: {}", args[1]);
    }
}
