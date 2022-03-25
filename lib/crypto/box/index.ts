import { Box } from '../../bindings';

const box = new Box();

export interface KeyPair {
	public_key: Uint8Array;
	secret_key: Uint8Array;
}

export interface CryptoBox {
	ciphertext: Uint8Array;
	mac: Uint8Array;
}

export const crypto_box_BEFORENMBYTES: number = box.cryptoBoxBeforenmbytes;
export const crypto_box_MACBYTES: number = box.cryptoBoxMacbytes;
export const crypto_box_MESSAGEBYTES_MAX: number = box.cryptoBoxMessagebytesMax;
export const crypto_box_NONCEBYTES: number = box.cryptoBoxNoncebytes;
export const crypto_box_PUBLICKEYBYTES: number = box.cryptoBoxPublickeybytes;
export const crypto_box_SEALBYTES: number = box.cryptoBoxSealbytes;
export const crypto_box_SECRETKEYBYTES: number = box.cryptoBoxSecretkeybytes;
export const crypto_box_SEEDBYTES: number = box.cryptoBoxSeedbytes;

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.precompute.html */
export const crypto_box_beforenm = (pk: Uint8Array, sk: Uint8Array): Uint8Array => box.crypto_box_beforenm(pk, sk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.seal_detached.html */
export const crypto_box_detached = (m: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): CryptoBox => box.crypto_box_detached(m, n, pk, sk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.seal.html */
export const crypto_box_easy = (m: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array => box.crypto_box_easy(m, n, pk, sk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.gen_keypair.html */
export const crypto_box_keypair = (): KeyPair => box.crypto_box_keypair();

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.gen_nonce.html */
export const crypto_box_nonce = (): Uint8Array => box.crypto_box_nonce();

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.open_detached.html */
export const crypto_box_open_detached = (c: Uint8Array, mac: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array =>
	box.crypto_box_open_detached(c, mac, n, pk, sk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.open_detached_precomputed.html */
export const crypto_box_open_detached_afternm = (c: Uint8Array, mac: Uint8Array, n: Uint8Array, k: Uint8Array): Uint8Array =>
	box.crypto_box_open_detached_afternm(c, mac, n, k);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.open.html */
export const crypto_box_open_easy = (c: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array =>
	box.crypto_box_open_easy(c, n, pk, sk);

/** @see https://docs.rs/dryoc/latest/dryoc/classic/crypto_box/fn.crypto_box_seal.html */
export const crypto_box_seal = (m: Uint8Array, pk: Uint8Array): Uint8Array => box.crypto_box_seal(m, pk);

/** @see https://docs.rs/dryoc/latest/dryoc/classic/crypto_box/fn.crypto_box_seal_open.html */
export const crypto_box_seal_open = (c: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array => box.crypto_box_seal_open(c, pk, sk);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.keypair_from_seed.html */
export const crypto_box_seed_keypair = (seed: Uint8Array): KeyPair => box.crypto_box_seed_keypair(seed);
