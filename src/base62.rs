const BASE62_TABLE: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

pub fn encode(mut num: u64) -> String {
    if num == 0 {
        return "0".to_owned();
    }
    let mut encoded = String::new();
    while num > 0 {
        let remainder = (num % 62) as usize;
        encoded.push(BASE62_TABLE.chars().nth(remainder).unwrap());
        num /= 62;
    }
    encoded.chars().rev().collect()
}

pub fn decode(s: &str) -> u64 {
    let mut num = 0;
    for (_, c) in s.chars().enumerate() {
        num = num * 62 + BASE62_TABLE.chars().position(|ch| ch == c).unwrap() as u64;
    }
    num
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn base62_test() {
        assert_eq!(decode(encode(0).as_str()), 0);
        assert_eq!(decode(encode(12345).as_str()), 12345);
        assert_eq!(decode(encode(0xffffffffff).as_str()), 0xffffffffff);
    }
}
