pub fn pad(message: Vec<u8>) -> Vec<u8> {
    let mut padded_message = message;
    let pad_length = 448 - (padded_message.len() % 512);
    if pad_length <= 0 {
        return padded_message;
    }
    padded_message.push(1);
    for _ in 1..pad_length {
        padded_message.push(0);
    }
    return padded_message;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 5);
    }
}
