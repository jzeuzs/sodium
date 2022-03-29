import { Sign } from '../bindings';
import type { KeyPair } from '../types';

const sign = new Sign();

export interface SignHash {
	state: BigUint64Array;
	count: BigUint64Array;
	buf: Uint8Array;
}

export interface SignState {
	hs: SignHash;
}

export const crypto_sign_BYTES: number = sign.cryptoSignBytes;
export const crypto_sign_MESSAGEBYTES_MAX: number = sign.cryptoSignMessagebytesMax;
export const crypto_sign_PUBLICKEYBYTES: number = sign.cryptoSignPublickeybytes;
export const crypto_sign_SECRETKEYBYTES: number = sign.cryptoSignSecretkeybytes;
export const crypto_sign_SEEDBYTES: number = sign.cryptoSignSeedbytes;

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.sign.html */
export const crypto_sign = (m: Uint8Array, sk: Uint8Array): Uint8Array => sign.crypto_sign(m, sk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.sign_detached.html */
export const crypto_sign_detached = (m: Uint8Array, sk: Uint8Array): Uint8Array => sign.crypto_sign_detached(m, sk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.to_curve25519_pk.html */
export const crypto_sign_ed25519_pk_to_curve25519 = (ed25519_pk: Uint8Array): Uint8Array => sign.crypto_sign_ed25519_pk_to_curve25519(ed25519_pk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.to_curve25519_sk.html */
export const crypto_sign_ed25519_sk_to_curve25519 = (ed25519_sk: Uint8Array): Uint8Array => sign.crypto_sign_ed25519_sk_to_curve25519(ed25519_sk);

/** @see https://docs.rs/libsodium-sys/latest/libsodium_sys/fn.crypto_sign_final_create.html */
export const crypto_sign_final_create = (state: SignState, sk: Uint8Array): Uint8Array => sign.crypto_sign_final_create(state, sk);

/** @see https://docs.rs/libsodium-sys/latest/libsodium_sys/fn.crypto_sign_final_verify.html */
export const crypto_sign_final_verify = (state: SignState, sig: Uint8Array, pk: Uint8Array): boolean => sign.crypto_sign_final_verify(state, sig, pk);

/** @see https://docs.rs/libsodium-sys/latest/libsodium_sys/fn.crypto_sign_init.html */
export const crypto_sign_init = (): SignState => sign.crypto_sign_init();

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.gen_keypair.html */
export const crypto_sign_keypair = (): KeyPair => sign.crypto_sign_keypair();

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.verify.html */
export const crypto_sign_open = (sm: Uint8Array, pk: Uint8Array): Uint8Array => sign.crypto_sign_open(sm, pk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.keypair_from_seed.html */
export const crypto_sign_seed_keypair = (seed: Uint8Array): KeyPair => sign.crypto_sign_seed_keypair(seed);

/** @see https://docs.rs/libsodium-sys/latest/libsodium_sys/fn.crypto_sign_update.html */
export const crypto_sign_update = (state: SignState, m: Uint8Array): SignState => sign.crypto_sign_update(state, m);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/sign/ed25519/fn.verify_detached.html */
export const crypto_sign_verify_detached = (sig: Uint8Array, m: Uint8Array, pk: Uint8Array): boolean => sign.crypto_sign_verify_detached(sig, m, pk);
