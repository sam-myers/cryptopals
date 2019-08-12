pub fn xor(bytes: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();

    for i in 0..bytes.len() {
        let key_idx = i % key.len();
        result.push(bytes[i] ^ key[key_idx]);
    }

    result
}
