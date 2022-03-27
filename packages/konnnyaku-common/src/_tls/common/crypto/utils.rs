pub fn f(x: u32, y: u32, z: u32) -> u32 {
    return (x & y) | (!x & z);
}

pub fn g(x: u32, y: u32, z: u32) -> u32 {
    return (x & z) | (y & !z);
}

pub fn h(x: u32, y: u32, z: u32) -> u32 {
    return x ^ y ^ z;
}

pub fn i(x: u32, y: u32, z: u32) -> u32 {
    return y ^ (x | !z);
}