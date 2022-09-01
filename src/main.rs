fn main() {
    let plaintext: String = String::from("Hello this is cool asdjfa;sjdfklajsdfl;kajsdkfjalskdjf");
    let mut ciphertext = String::new();

    ciphertext = xor_shift(plaintext);
}

fn xor_shift(text: String) -> String {
    let bytes = text.as_bytes();
    let mut ciphertext = String::new();

    for (i, byte) in bytes.iter().enumerate() {
        let byte1 = byte ^ byte*2 ^ i as u8;
        ciphertext.push(byte1 as char);
    }
    ciphertext
}

fn shifter(text: String) -> String {
    let bytes = text.as_bytes();
    let mut ciphertext = String::new();

    for (i, byte) in bytes.iter().enumerate() {
        
        ciphertext.push(byte1 as char);
    }
    ciphertext
}