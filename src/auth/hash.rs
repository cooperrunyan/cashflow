use crate::config::CONFIG;
use argon2rs::{Argon2, Variant};

const PASSES: u32 = 4; //    Default 3
const LANES: u32 = 2; //     Default 1
const KIB: u32 = 4096; //    Default 4096
const LENGTH: usize = 64; // Default 32

const VARIANT: Variant = Variant::Argon2i;

lazy_static! {
    static ref HASHER: Argon2 = get_hasher();
}

pub fn hash(plaintext: impl ToString) -> String {
    let mut out = [0; LENGTH];

    HASHER.hash(
        &mut out,
        &plaintext.to_string().as_bytes(),
        &CONFIG.hash_salt.as_bytes(),
        &CONFIG.hash_key.as_bytes(),
        &[],
    );

    out.iter().map(|b| format!("{:02x}", b)).collect()
}

// This should prevent against time-attacks
// input and reference *should* always be 64 bytes (if it is valid)
pub fn check_hash(input: impl ToString, reference: impl ToString) -> bool {
    let mut pass = true;

    let input = input.to_string();
    let input = input.as_bytes();
    let reference = reference.to_string();
    let reference = reference.as_bytes();

    for i in 0..(LENGTH - 1) {
        let byt_in = match input.get(i) {
            None => {
                pass = false;
                &0
            }
            Some(b) => b,
        };

        let byt_ref = match reference.get(i) {
            None => {
                pass = false;
                &0
            }
            Some(b) => b,
        };

        if !byt_in.eq(byt_ref) {
            pass = false
        }
    }

    pass
}

fn get_hasher() -> Argon2 {
    Argon2::new(PASSES, LANES, KIB, VARIANT).expect("Failed to create hasher")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static HASH_INPUT1: &str = "test@test.com";
    static HASH_INPUT2: &str = "test@test.cob";
    static HASH_INPUT3: &str = "te";

    #[actix_rt::test]
    async fn it_hashes() {
        let h = hash(HASH_INPUT1);
        assert_ne!(HASH_INPUT1, h);
    }

    #[actix_rt::test]
    async fn it_returns_same_hash() {
        let h1 = hash(HASH_INPUT1);
        let h2 = hash(HASH_INPUT1);
        assert_eq!(h1, h2);
    }

    #[actix_rt::test]
    async fn it_checks_hashes() {
        let h1 = hash(HASH_INPUT1);
        let h2 = hash(HASH_INPUT1);

        let check = check_hash(h1, h2);

        assert_eq!(check, true);
    }

    #[actix_rt::test]
    async fn it_fails_hashes() {
        let h1 = hash(HASH_INPUT1);
        let h2 = hash(HASH_INPUT2);

        let check = check_hash(h1, h2);

        assert_eq!(check, false);
    }

    #[actix_rt::test]
    async fn it_fails_hashes_of_diff_len() {
        let h1 = hash(HASH_INPUT1);
        let h2 = hash(HASH_INPUT3);

        let check = check_hash(h1, h2);

        assert_eq!(check, false);
    }
}
