pub fn bytes_to_base58(input: &[u8]) -> String {
    bs58::encode(input).into_string()
}
