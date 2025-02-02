use std::time::{SystemTime, UNIX_EPOCH};

fn simple_random_u32() -> u32 {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let nanos = duration.as_nanos() as u32; // Usa i nanosecondi come seed

    (nanos.wrapping_mul(1664525).wrapping_add(1013904223)) // LCG (Linear Congruential Generator)
}

pub fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let charset_len = CHARSET.len();
    
    (0..length)
        .map(|_| CHARSET[simple_random_u32() as usize % charset_len] as char)
        .collect()
}

#[warn(dead_code)]
pub fn random_u32_range(min: u32, max: u32) -> u32 {
    min + (simple_random_u32() % (max - min + 1))
}

pub fn random_u64() -> u64 {
    let low = simple_random_u32() as u64;
    let high = simple_random_u32() as u64;
    (high << 32) | low // Combina due u32 per ottenere un u64
}

#[warn(dead_code)]
pub fn random_usize() -> usize {
    simple_random_u32() as usize
}