use cife_rs::abe::dippe::*;

pub struct KeyMaterial {
    pub dippe: Dippe,
    pub public: PublicKey,
    pub private: PrivateKey,
}

impl KeyMaterial {
    pub fn load_from_storage() -> Option<Self> {
        None
    }

    pub fn generate_and_persist(seed: &[u8], assumptions: usize) -> Self {
        let dippe = Dippe::new(seed, assumptions);
        let mut rng = rand::thread_rng();
        let (public, private) = dippe.generate_key_pair(&mut rng);
        let km = KeyMaterial {
            dippe,
            public,
            private,
        };

        km
    }
}
