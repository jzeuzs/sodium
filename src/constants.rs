#![allow(non_upper_case_globals)]

#[napi]
pub mod constants {
    use crate::{create_constant, create_constant_from_dryoc};

    create_constant!(crypto_aead_chacha20poly1305_ABYTES, u32);
    create_constant!(crypto_aead_chacha20poly1305_ietf_ABYTES, u32);
    create_constant!(crypto_aead_chacha20poly1305_IETF_ABYTES, u32);
    create_constant!(crypto_aead_chacha20poly1305_ietf_KEYBYTES, u32);
    create_constant_from_dryoc!(
        crypto_aead_chacha20poly1305_ietf_MESSAGEBYTES_MAX,
        CRYPTO_AEAD_CHACHA20POLY1305_IETF_MESSAGEBYTES_MAX,
        u64
    );
    create_constant_from_dryoc!(
        crypto_aead_chacha20poly1305_IETF_MESSAGEBYTES_MAX,
        CRYPTO_AEAD_CHACHA20POLY1305_IETF_MESSAGEBYTES_MAX,
        u64
    );
    create_constant!(crypto_aead_chacha20poly1305_ietf_NPUBBYTES, u32);
    create_constant!(crypto_aead_chacha20poly1305_IETF_NPUBBYTES, u32);
    create_constant!(crypto_aead_chacha20poly1305_ietf_NSECBYTES, u32);
    create_constant!(crypto_aead_chacha20poly1305_IETF_NSECBYTES, u32);
    create_constant!(crypto_aead_chacha20poly1305_KEYBYTES, u32);
}
