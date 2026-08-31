pub fn fingerprint(ciphertext: &[u8]) -> String {
    blake3::hash(ciphertext).to_hex().to_string()
}
