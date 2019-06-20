use std::str;

fn main() {
    let byte_escape = "I'm writing \x52\x75\x73\x74!";// Rust
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    let unicode_codepoint = "\u{211D}";// ℝ
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";
    println!("Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name);

    let long_string = "String literals
                        can span multiplelines.
                        The linebreak and identation here ->\
                        <- can be escaped too!";
    println!("{}", long_string);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";
    println!("{}", raw_str);
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);
    let longer_delimiter = r###"A string with "# in it. And even"##!"###;
    println!("{}", longer_delimiter);

    let bytestring: &[u8; 20] = b"this is a bytestring";
    println!("A bytestring: {:?}", bytestring);
    // [116, 104, 105, 115, 32, 105, ...]
    let escaped = b"\x52\x75\x73\x74 as bytes";
    println!("Some escaped bytes: {:?}", escaped);
    //let escaped2 = b"\u{211D} is not allowed";// error: unicode escape sequences cannot be used as a byte or in a byte string

    let raw_bytestring = br"\u{211D} is not escaped here";
    println!("{:?}", raw_bytestring);
    if let Ok(my_str) = str::from_utf8(raw_bytestring) {
        println!("And the same as text: '{}'", my_str);// \u{211D} is not escaped here
    }

    let _quoted = br#"You can also use "fancier" formatting, \
                    like with normal raw strings"#;

    let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82";// ようこそ
    match str::from_utf8(shift_jis) {
        Ok(my_str) => println!("Conversion successful: '{}'", my_str),
        Err(e) => println!("Conversion failed: {:?}", e),// Utf8Error { valid_up_to: 0, error_len: Some(1) }
    }
}