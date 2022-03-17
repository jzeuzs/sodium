import { Box } from '../../bindings';

const box = new Box();

export interface KeyPair {
	publicKey: Uint8Array;
	secretKey: Uint8Array;
}

/**
 * Number of bytes in the authenticator tag of an encrypted message i.e. the number of bytes by which the ciphertext is larger than the plaintext.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/constant.MACBYTES.html
 */
export const crypto_box_MACBYTES: number = box.macbytes;

/**
 * Number of bytes in a Nonce.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/constant.NONCEBYTES.html
 */
export const crypto_box_NONCEBYTES: number = box.noncebytes;

/**
 * Number of bytes in a PrecomputedKey.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/constant.PRECOMPUTEDKEYBYTES.html
 */
export const crypto_box_PRECOMPUTEDKEYBYTES: number = box.precomputedkeybytes;

/**
 * Randomly generates a secret key and a corresponding public key.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.gen_keypair.html
 */
export const crypto_box_gen_keypair = (): KeyPair => box.gen_keypair();

/**
 * Randomly generates a nonce.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.gen_nonce.html
 */
export const crypto_box_gen_nonce = (): Uint8Array => box.gen_nonce();

/**
 * Verifies and decrypts a ciphertext using the receiver’s secret key, the senders public key, and a nonce. It returns a plaintext.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.open.html
 */
export const crypto_box_open = (cipher_text: Uint8Array, nonce: Uint8Array, public_key: Uint8Array, secret_key: Uint8Array): Uint8Array =>
	box.open(cipher_text, nonce, public_key, secret_key);

/**
 * Verifies and decrypts a ciphertext using the receiver’s secret key, the senders public key, and a nonce. The ciphertext is decrypted in place, so if this function is successful it will contain the plaintext.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.open_detached.html
 */
export const crypto_box_open_detached = (
	cipher_text: Uint8Array,
	mac: Uint8Array,
	nonce: Uint8Array,
	public_key: Uint8Array,
	secret_key: Uint8Array
): Uint8Array => box.open_detached(cipher_text, mac, nonce, public_key, secret_key);

/**
 * Verifies and decrypts a ciphertext using a precomputed key and a nonce. The ciphertext is decrypted in place, so if this function is successful it will contain the plaintext.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.open_detached_precomputed.html
 */
export const crypto_box_open_detached_precomputed = (
	cipher_text: Uint8Array,
	mac: Uint8Array,
	nonce: Uint8Array,
	precomputed_key: Uint8Array
): Uint8Array => box.open_detached_precomputed(cipher_text, mac, nonce, precomputed_key);

/**
 * Verifies and decrypts a ciphertext using a precomputed key and a nonce. It returns a plaintext.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.open_precomputed.html
 */
export const crypto_box_open_precomputed = (cipher_text: Uint8Array, nonce: Uint8Array, precomputed_key: Uint8Array): Uint8Array =>
	box.open_precomputed(cipher_text, nonce, precomputed_key);

/**
 * Computes an intermediate key that can be used by seal_precomputed() and open_precomputed().
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.precompute.html
 */
export const crypto_box_precompute = (public_key: Uint8Array, secret_key: Uint8Array): Uint8Array => box.precompute(public_key, secret_key);

/**
 * Encrypts and authenticates a message using the senders secret key, the receivers public key and a nonce. It returns a ciphertext.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.seal.html
 */
export const crypto_box_seal = (message: Uint8Array, nonce: Uint8Array, public_key: Uint8Array, secret_key: Uint8Array): Uint8Array =>
	box.seal(message, nonce, public_key, secret_key);

/**
 * Encrypts and authenticates a message using the senders secret key, the receivers public key and a nonce. The message is encrypted in place, so after this function returns it will contain the ciphertext. The detached authentication tag is returned by value.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.seal_detached.html
 */
export const crypto_box_seal_detached = (message: Uint8Array, nonce: Uint8Array, public_key: Uint8Array, secret_key: Uint8Array): Uint8Array =>
	box.seal_detached(message, nonce, public_key, secret_key);

/**
 * Encrypts and authenticates a message using a precomputed key and a nonce. The message is encrypted in place, so after this function returns it will contain the ciphertext. The detached authentication tag is returned by value.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.seal_detached_precomputed.html
 */
export const crypto_box_seal_detached_precomputed = (message: Uint8Array, nonce: Uint8Array, precomputed_key: Uint8Array): Uint8Array =>
	box.seal_detached_precomputed(message, nonce, precomputed_key);

/**
 * Encrypts and authenticates a message using a precomputed key, and a nonce. It returns a ciphertext.
 * @see https://docs.rs/sodiumoxide/0.2.7/sodiumoxide/crypto/box_/curve25519xsalsa20poly1305/fn.seal_precomputed.html
 */
export const crypto_box_seal_precomputed = (message: Uint8Array, nonce: Uint8Array, precomputed_key: Uint8Array): Uint8Array =>
	box.seal_precomputed(message, nonce, precomputed_key);
