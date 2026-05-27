use rand::Rng;

pub fn generate_random_string(size: usize) -> String {
    let mut rng = rand::thread_rng();
    let chars = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    (0..size)
        .map(|_| chars[rng.gen_range(0..chars.len())] as char)
        .collect()
}
