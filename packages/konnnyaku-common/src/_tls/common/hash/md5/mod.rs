mod append_length;
mod init_md_buffer;
mod pad;
mod utils;

use append_length::append_length;
use init_md_buffer::init_md_buffer;
use pad::pad;
use utils::{f, g, getT, h, i};

pub fn md5(message: Vec<u8>) -> [u8; 16] {
    let message = message;
    let message_length = message.len() * 8;
    let message = pad(message);
    let message = append_length(message, message_length as u64);

    let mut message_u32 = vec![] as Vec<u32>;

    // read bytes with little-eindian
    for x in 0..(message.len() / (32 / 8)) {
        message_u32.push(
            ((message[x * 4 + 3] as u32) << 24)
                | ((message[x * 4 + 2] as u32) << 16)
                | ((message[x * 4 + 1] as u32) << 8)
                | (message[x * 4 + 0] as u32),
        );
    }

    let message = message_u32;

    let (mut a, mut b, mut c, mut d) = init_md_buffer();
    let t = getT();
    for x in 0..(message.len() / 16) {
        let aa = a;
        let bb = b;
        let cc = c;
        let dd = d;

        let block = &message[x..(x + 16)];
        let x = block;

        macro_rules! op1 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(f($b, $c, $d))
                        .wrapping_add(x[$k].into())
                        .wrapping_add(t[$i - 1]))
                    .rotate_left($s),
                )
            };
        }

        op1!(a, b, c, d, 0, 7, 1);
        op1!(d, a, b, c, 1, 12, 2);
        op1!(c, d, a, b, 2, 17, 3);
        op1!(b, c, d, a, 3, 22, 4);

        op1!(a, b, c, d, 4, 7, 5);
        op1!(d, a, b, c, 5, 12, 6);
        op1!(c, d, a, b, 6, 17, 7);
        op1!(b, c, d, a, 7, 22, 8);

        op1!(a, b, c, d, 8, 7, 9);
        op1!(d, a, b, c, 9, 12, 10);
        op1!(c, d, a, b, 10, 17, 11);
        op1!(b, c, d, a, 11, 22, 12);

        op1!(a, b, c, d, 12, 7, 13);
        op1!(d, a, b, c, 13, 12, 14);
        op1!(c, d, a, b, 14, 17, 15);
        op1!(b, c, d, a, 15, 22, 16);

        /* Round 2. Let [abcd k s i] denote the operation
        a = b + ((a + G(b,c,d) + X[k] + T[i]) <<< s). */
        macro_rules! op2 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(g($b, $c, $d))
                        .wrapping_add(x[$k].into())
                        .wrapping_add(t[$i - 1]))
                    .rotate_left($s),
                )
            };
        }

        /* do the following 16 operations. */
        op2!(a, b, c, d, 1, 5, 17);
        op2!(d, a, b, c, 6, 9, 18);
        op2!(c, d, a, b, 11, 14, 19);
        op2!(b, c, d, a, 0, 20, 20);

        op2!(a, b, c, d, 5, 5, 21);
        op2!(d, a, b, c, 10, 9, 22);
        op2!(c, d, a, b, 15, 14, 23);
        op2!(b, c, d, a, 4, 20, 24);

        op2!(a, b, c, d, 9, 5, 25);
        op2!(d, a, b, c, 14, 9, 26);
        op2!(c, d, a, b, 3, 14, 27);
        op2!(b, c, d, a, 8, 20, 28);

        op2!(a, b, c, d, 13, 5, 29);
        op2!(d, a, b, c, 2, 9, 30);
        op2!(c, d, a, b, 7, 14, 31);
        op2!(b, c, d, a, 12, 20, 32);

        /* Round 3. Let [abcd k s t] denote the operation
        a = b + ((a + H(b,c,d) + X[k] + T[i]) <<< s). */
        macro_rules! op3 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(h($b, $c, $d))
                        .wrapping_add(x[$k].into())
                        .wrapping_add(t[$i - 1]))
                    .rotate_left($s),
                )
            };
        }

        /* do the following 16 operations. */
        op3!(a, b, c, d, 5, 4, 33);
        op3!(d, a, b, c, 8, 11, 34);
        op3!(c, d, a, b, 11, 16, 35);
        op3!(b, c, d, a, 14, 23, 36);

        op3!(a, b, c, d, 1, 4, 37);
        op3!(d, a, b, c, 4, 11, 38);
        op3!(c, d, a, b, 7, 16, 39);
        op3!(b, c, d, a, 10, 23, 40);

        op3!(a, b, c, d, 13, 4, 41);
        op3!(d, a, b, c, 0, 11, 42);
        op3!(c, d, a, b, 3, 16, 43);
        op3!(b, c, d, a, 6, 23, 44);

        op3!(a, b, c, d, 9, 4, 45);
        op3!(d, a, b, c, 12, 11, 46);
        op3!(c, d, a, b, 15, 16, 47);
        op3!(b, c, d, a, 2, 23, 48);

        /* Round 4. Let [abcd k s t] denote the operation
        a = b + ((a + I(b,c,d) + X[k] + T[i]) <<< s). */
        macro_rules! op4 {
            ($a:ident,$b:ident,$c:ident,$d:ident,$k:expr,$s:expr,$i:expr) => {
                $a = $b.wrapping_add(
                    ($a.wrapping_add(i($b, $c, $d))
                        .wrapping_add(x[$k].into())
                        .wrapping_add(t[$i - 1]))
                    .rotate_left($s),
                )
            };
        }

        /* do the following 16 operations. */
        op4!(a, b, c, d, 0, 6, 49);
        op4!(d, a, b, c, 7, 10, 50);
        op4!(c, d, a, b, 14, 15, 51);
        op4!(b, c, d, a, 5, 21, 52);

        op4!(a, b, c, d, 12, 6, 53);
        op4!(d, a, b, c, 3, 10, 54);
        op4!(c, d, a, b, 10, 15, 55);
        op4!(b, c, d, a, 1, 21, 56);

        op4!(a, b, c, d, 8, 6, 57);
        op4!(d, a, b, c, 15, 10, 58);
        op4!(c, d, a, b, 6, 15, 59);
        op4!(b, c, d, a, 13, 21, 60);

        op4!(a, b, c, d, 4, 6, 61);
        op4!(d, a, b, c, 11, 10, 62);
        op4!(c, d, a, b, 2, 15, 63);
        op4!(b, c, d, a, 9, 21, 64);

        a = a.wrapping_add(aa);
        b = b.wrapping_add(bb);
        c = c.wrapping_add(cc);
        d = d.wrapping_add(dd);
    }

    let (a, b, c, d) = (
        a.swap_bytes(),
        b.swap_bytes(),
        c.swap_bytes(),
        d.swap_bytes(),
    );

    [
        (a >> 24) as u8,
        (a >> 16) as u8,
        (a >> 8) as u8,
        a as u8,
        (b >> 24) as u8,
        (b >> 16) as u8,
        (b >> 8) as u8,
        b as u8,
        (c >> 24) as u8,
        (c >> 16) as u8,
        (c >> 8) as u8,
        c as u8,
        (d >> 24) as u8,
        (d >> 16) as u8,
        (d >> 8) as u8,
        d as u8,
    ]
}

pub fn to_hex(bytes: [u8; 16]) -> String {
    bytes
        .iter()
        .map(|n| format!("{:02x}", n))
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let message = "abc".as_bytes();
        let result = md5(message.into());
        assert_eq!(to_hex(result), "900150983cd24fb0d6963f7d28e17f72");
    }
}
