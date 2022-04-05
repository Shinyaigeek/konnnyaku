pub fn pad(message: Vec<u8>) -> Vec<u8> {
    let mut padded_message = message;
    let pad_length = (448 - (padded_message.len() * 8 % 512)) / 8;
    if pad_length <= 0 {
        return padded_message;
    }
    // the first bit is 1
    padded_message.push(0b10000000);
    for _ in 1..pad_length {
        padded_message.push(0_u8);
    }
    return padded_message;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let padded = pad(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!((padded.len() * 8) % 512, 448);
        assert_eq!(padded[0..10], [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(padded[10..11], [0b10000000]);
        assert_eq!(padded[11..padded.len()], [0; 45]);
    }
}
