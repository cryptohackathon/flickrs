use cife_rs::abe::dippe::*;
use rand::*;
use ring::aead::{Aad, LessSafeKey, Nonce, UnboundKey, CHACHA20_POLY1305};
use tiny_keccak::{Hasher, Sha3};

/// Takes a byte string and encrypts and authenticates it for storage on the flick-rs server.
pub fn seal<R: RngCore + CryptoRng>(
    rng: &mut R,
    dippe: &Dippe,
    authority: &PublicKey,
    bytes: &[u8],
    policy: &[usize],
    attributes: usize,
) -> Vec<u8> {
    // Generate the key
    let sealing_key_gt: Gt = rng.gen();
    let sealing_key = {
        let mut sealing_key_bytes = [0u8; 32];

        let mut hash = Sha3::v256();
        hash.update(&sealing_key_gt.into_bytes());
        hash.finalize(&mut sealing_key_bytes);
        sealing_key_bytes
    };

    let sealing_key = UnboundKey::new(&CHACHA20_POLY1305, &sealing_key).unwrap();
    // We don't have nonce reuse, we can use LessSafeKey.
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    let sealing_key = LessSafeKey::new(sealing_key);

    // Format: 64 bytes for the padded encryption key as Gt element,
    //         + cipher text
    //         + tag

    // First, the DIPPE ABE
    let ep = dippe.create_conjunction_policy_vector(rng, attributes, policy);
    let pks: Vec<_> = (0..(attributes + 1)).map(|_| authority).collect();
    let encrypted_sealing_key_gt = dippe.encrypt(rng, &ep, sealing_key_gt, &pks);
    let ciphertext_len = encrypted_sealing_key_gt.bytes_len();

    let mut output = vec![0u8; ciphertext_len + bytes.len() + CHACHA20_POLY1305.tag_len()];
    output[..ciphertext_len].clone_from_slice(&encrypted_sealing_key_gt.into_bytes());

    let buf = &mut output[ciphertext_len..];
    // Then, the symmetric encryption
    let mut text = &mut buf[..bytes.len()];
    text.clone_from_slice(bytes);
    let tag = sealing_key
        .seal_in_place_separate_tag(nonce, Aad::empty(), &mut text)
        .unwrap();
    drop(text);
    buf[bytes.len()..].clone_from_slice(tag.as_ref());

    output
}

/// Takes a byte string and private key, and decrypts and deauthenticates it.
///
/// Inverse operation of [`seal`].
pub fn open(
    dippe: &Dippe,
    attributes: usize,
    av: &AttributeVector,
    upk: &UserPrivateKey,
    gid: &[u8],
    sealed: &[u8],
) -> Option<Vec<u8>> {
    let sealing_key = &sealed[..CipherText::len_for(attributes + 1, 2)];
    let ciphertext = &sealed[CipherText::len_for(attributes + 1, 2)..];

    // ABE decrypt
    let sealing_key = CipherText::from_bytes(dippe.assumptions(), av.len(), sealing_key);
    let gt = dippe.decrypt(upk, sealing_key, av, gid);
    let sealing_key = {
        let mut sealing_key_bytes = [0u8; 32];
        let mut hash = Sha3::v256();
        hash.update(&gt.into_bytes());
        hash.finalize(&mut sealing_key_bytes);
        sealing_key_bytes
    };

    // Symmetric decrypt
    let sealing_key = UnboundKey::new(&CHACHA20_POLY1305, &sealing_key).unwrap();
    // We don't have nonce reuse, we can use LessSafeKey.
    let nonce = Nonce::assume_unique_for_key([0u8; 12]);
    let sealing_key = LessSafeKey::new(sealing_key);

    let mut result = Vec::from(ciphertext);
    let res = sealing_key
        .open_in_place(nonce, Aad::empty(), &mut result)
        .ok()?;
    let len = res.len();
    result.truncate(len);

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn end_to_end_seal() {
        let mut rng = rand::thread_rng();
        let dippe = Dippe::randomized(&mut rng, 2);

        let plaintext = b"hello, world!!";

        let (alice_pub, alice_priv) = dippe.generate_key_pair(&mut rng);

        let attribs = 1;
        let carol_policy = &[0];
        let carol_policy = dippe.create_attribute_vector(attribs, carol_policy);
        let gid = b"Carol";

        // this is done by the server
        let carol_upk = {
            let vec_len = attribs + 1;
            let mut usks = Vec::with_capacity(vec_len);
            for j in 0..vec_len {
                usks.push(dippe.generate_user_private_key_part(
                    &alice_priv,
                    j,
                    &[&alice_pub, &alice_pub],
                    gid,
                    &carol_policy,
                ));
            }
            let upk: Result<UserPrivateKeySlice, _> = usks.into_iter().collect();
            UserPrivateKey::try_from(upk.unwrap()).unwrap()
        };
        // end done by the server

        let sealed = seal(&mut rng, &dippe, &alice_pub, plaintext, &[0], attribs);
        let opened = open(&dippe, attribs, &carol_policy, &carol_upk, gid, &sealed);
        assert_eq!(opened.as_ref().unwrap(), plaintext);
    }
}
