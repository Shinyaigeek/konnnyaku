// pub fn append_length(message: Vec<u8>, msg_length: u64) -> Vec<u8> {
//     let mut added_message = message;
// }

// fn convert_u64_to_u8s(msg_length: u64) -> [u8; 8] {
//     [
//         msg_length as u8,
//         (msg_length >> 8) as u8,
//         (msg_length >> 16) as u8,
//         (msg_length >> 24) as u8,
//         (msg_length >> 32) as u8,
//         (msg_length >> 40) as u8,
//         (msg_length >> 48) as u8,
//         (msg_length >> 56) as u8,
//     ]
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let padded = pad(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
//         assert_eq!(padded.len() % 512, 448);
//         assert_eq!(padded[0..10], [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
//         assert_eq!(padded[10..11], [1]);
//         assert_eq!(padded[11..padded.len()], [0; 437]);
//     }
// }
