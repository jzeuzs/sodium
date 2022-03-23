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

/**
 * Computes a shared secret for the given `public_key` and `private_key`. Resulting shared secret can be used with the precalculation interface.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_beforenm.html crypto_box::crypto_box_beforenm}
 */
export const crypto_box_beforenm = (public_key: Uint8Array, secret_key: Uint8Array): Uint8Array => box.crypto_box_beforenm(public_key, secret_key);

/**
 * Detached variant of {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_easy.html crypto_box_easy}.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_detached.html crypto_box::crypto_box_detached}
 */
export const crypto_box_detached = (
	message: Uint8Array,
	nonce: Uint8Array,
	recipient_public_key: Uint8Array,
	sender_secret_key: Uint8Array
): CryptoBox => box.crypto_box_detached(message, nonce, recipient_public_key, sender_secret_key);

/**
 * Encrypts `message` with recipient’s public key `recipient_public_key`, sender’s secret key `sender_secret_key`, and `nonce`. The result is placed into `ciphertext` which must be the length of the message plus {@link https://docs.rs/dryoc/0.3.12/dryoc/constants/constant.CRYPTO_BOX_MACBYTES.html CRYPTO_BOX_MACBYTES} bytes, for the message tag.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_easy.html crypto_box::crypto_box_easy}
 */
export const crypto_box_easy = (
	message: Uint8Array,
	nonce: Uint8Array,
	recipient_public_key: Uint8Array,
	sender_secret_key: Uint8Array
): Uint8Array => box.crypto_box_easy(message, nonce, recipient_public_key, sender_secret_key);

/**
 * Generates a public/secret key pair using OS provided data.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_keypair.html crypto_box::crypto_box_keypair}
 */
export const crypto_box_keypair = (): KeyPair => box.crypto_box_keypair();

/**
 * Generates a nonce.
 */
export const crypto_box_nonce = (): Uint8Array => box.crypto_box_nonce();

/**
 * Detached variant of {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_open_easy.html crypto_box_open_easy}.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_open_detached.html crypto_box::crypto_box_open_detached}
 */
export const crypto_box_open_detached = (
	mac: Uint8Array,
	ciphertext: Uint8Array,
	nonce: Uint8Array,
	recipient_public_key: Uint8Array,
	sender_secret_key: Uint8Array
): Uint8Array => box.crypto_box_open_detached(mac, ciphertext, nonce, recipient_public_key, sender_secret_key);

/**
 * Precalculation variant of {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_open_easy.html crypto_box_open_easy}.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_open_detached_afternm.html crypto_box::crypto_box_open_detached_afternm}
 */
export const crypto_box_open_detached_afternm = (mac: Uint8Array, ciphertext: Uint8Array, nonce: Uint8Array, key: Uint8Array): Uint8Array =>
	box.crypto_box_open_detached_afternm(mac, ciphertext, nonce, key);

/**
 * Decrypts `ciphertext` with recipient’s secret key `recipient_secret_key` and sender’s public key `sender_public_key` using `nonce`.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_open_easy.html crypto_box::crypto_box_open_easy}
 */
export const crypto_box_open_easy = (
	ciphertext: Uint8Array,
	nonce: Uint8Array,
	sender_public_key: Uint8Array,
	recipient_secret_key: Uint8Array
): Uint8Array => box.crypto_box_open_easy(ciphertext, nonce, sender_public_key, recipient_secret_key);

/**
 * Encrypts `message` with recipient’s public key `recipient_public_key`, using an ephemeral keypair and nonce. The length of `ciphertext` must be the length of the message plus {@link https://docs.rs/dryoc/0.3.12/dryoc/constants/constant.CRYPTO_BOX_SEALBYTES.html CRYPTO_BOX_SEALBYTES} bytes for the message tag and ephemeral public key.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_seal.html crypto_box::crypto_box_seal}
 */
export const crypto_box_seal = (message: Uint8Array, recipient_public_key: Uint8Array): Uint8Array =>
	box.crypto_box_seal(message, recipient_public_key);

/**
 * Decrypts a sealed box from `ciphertext` with recipient’s secret key `recipient_secret_key`, placing the result into `message`. The nonce and public are derived from `ciphertext`. `message` length should be equal to the length of `ciphertext` minus {@link https://docs.rs/dryoc/0.3.12/dryoc/constants/constant.CRYPTO_BOX_SEALBYTES.html CRYPTO_BOX_SEALBYTES} bytes for the message tag and ephemeral public key.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_seal_open.html crypto_box::crypto_box_seal_open}
 */
export const crypto_box_seal_open = (ciphertext: Uint8Array, recipient_public_key: Uint8Array, recipient_secret_key: Uint8Array): Uint8Array =>
	box.crypto_box_seal_open(ciphertext, recipient_public_key, recipient_secret_key);

/**
 * Deterministically derives a keypair from `seed`, which can be of arbitrary length.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/classic/crypto_box/fn.crypto_box_seed_keypair.html crypto_box::crypto_box_seed_keypair}
 */
export const crypto_box_seed_keypair = (seed: Uint8Array): KeyPair => box.crypto_box_seed_keypair(seed);
