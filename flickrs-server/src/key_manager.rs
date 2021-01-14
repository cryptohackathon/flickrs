use std::path::PathBuf;

use cife_rs::abe::dippe::*;

pub struct KeyMaterial {
    pub dippe: Dippe,
    pub public: PublicKey,
    pub private: PrivateKey,
}

impl KeyMaterial {
    fn paths() -> (PathBuf, PathBuf) {
        let base = std::env::var("KEY_STORE_PATH").expect("Key store path not set!");
        let base = std::path::Path::new(&base);
        if let Err(e) = std::fs::create_dir_all(base) {
            log::warn!("{}", e);
        }

        let private_path = base.join("private.key");
        let public_path = base.join("public.key");
        (private_path, public_path)
    }

    pub fn load_from_storage(seed: &[u8], assumptions: usize) -> Option<Self> {
        use std::fs;
        let (prv, pbk) = Self::paths();

        // Read
        let (prv, pbk) = match (fs::read_to_string(prv), fs::read_to_string(pbk)) {
            (Ok(prv), Ok(pbk)) => Some((prv, pbk)),
            (Err(e1), Err(e2)) => {
                log::warn!("Could not read keys from storage: {} and {}", e1, e2);
                None
            }
            (Err(e), _) | (_, Err(e)) => {
                log::error!("Could only read one key from storage; {}; resetting!", e);
                None
            }
        }?;

        // Decode
        let prv = serde_json::from_str(&prv)
            .map_err(|e| log::error!("Loading private key: {}", e))
            .ok()?;
        let pbk = serde_json::from_str(&pbk)
            .map_err(|e| log::error!("Loading public key: {}", e))
            .ok()?;

        Some(KeyMaterial {
            dippe: Dippe::new(seed, assumptions),
            public: pbk,
            private: prv,
        })
    }

    pub fn generate_and_persist(seed: &[u8], assumptions: usize) -> Self {
        use std::fs;

        let dippe = Dippe::new(seed, assumptions);
        let mut rng = rand::thread_rng();
        let (public, private) = dippe.generate_key_pair(&mut rng);
        let km = KeyMaterial {
            dippe,
            public,
            private,
        };
        let (prv, pbk) = Self::paths();

        // Persist
        match (
            fs::write(
                prv,
                serde_json::to_string(&km.private).expect("infallible serialization"),
            ),
            fs::write(
                pbk,
                serde_json::to_string(&km.public).expect("infallible serialization"),
            ),
        ) {
            (Ok(()), Ok(())) => log::info!("Persisted new keys."),
            (r1, r2) => log::info!("Error persisting new keys. Priv: {:?}, pub: {:?}", r1, r2),
        };

        km
    }
}
