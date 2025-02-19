use serde::Deserialize;

pub fn gen_random(len: usize) -> Vec<u8> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..len).map(|_| rng.gen()).collect()
}

#[derive(Deserialize)]
pub struct BackupRequest {
    pub encryption_key: String,
}

#[derive(Deserialize)]
pub struct RestoreRequest {
    pub encryption_key: String,
    pub backup_data: Vec<u8>,
}