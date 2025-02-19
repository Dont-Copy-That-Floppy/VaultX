use rand::Rng;

pub fn generate_key() -> [u8; 32] {
    rand::thread_rng().gen()
}
