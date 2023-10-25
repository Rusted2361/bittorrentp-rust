use serde_json::{json, Value};
use std::env;

// Available if you need it!
// use serde_bencode
#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Result<Value, String> {
    // If encoded_value starts with a digit, it's a string
    let first_char = encoded_value.chars().next().ok_or("Empty input")?; 
    if first_char.is_digit(10) {
        // Example: "5:hello" -> "hello"
        let colon_index = encoded_value.find(':').ok_or("Invalid format")?;
        let number_string = &encoded_value[..colon_index];
        let number = number_string.parse::<i64>().map_err(|e| format!("Invalid number format: {}", e))?;
        let string = &encoded_value[colon_index + 1..colon_index + 1 + number as usize];
        Ok(Value::String(string.to_string()))
    } else if first_char == 'i' {
        // Example: i52e -> 52
        let end_index = encoded_value.len() - 1;
        let number = &encoded_value[1..end_index].parse::<i64>().unwrap();
        Ok(json!(number))
    }
    else {
        Err("Unhandled encoded value".to_string())
    }
}

// Usage: your_bittorrent.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        match decode_bencoded_value(encoded_value) {
            Ok(decoded_value) => println!("{}", decoded_value.to_string()),
            Err(err) => eprintln!("Error: {}", err),
        }
    } 
    else {
        println!("unknown command: {}", args[1])
    }
}
