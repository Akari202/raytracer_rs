/// An implementation of the fast inverse square root function from quake 3
pub fn q_rsqrt(n: f32) -> f32 {
    let three_halfs = 1.5;
    let x2 = n * 0.5;
    let mut y = n;
    let mut i: i32 = y.to_bits() as i32;
    i = 0x5f3759df - (i >> 1); // what the fuck?
    y = f32::from_bits(i as u32);
    y = y * (three_halfs - (x2 * y * y)); // 1st iteration
    // y = y * (three_halfs - (x2 * y * y)); // 2nd iteration, can be removed
    y
}
