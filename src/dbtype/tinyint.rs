pub struct TinyInt {
    pub value: i8,
}
impl TinyInt {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            value: i8::from_be_bytes([bytes[0]]),
        }
    }
}
