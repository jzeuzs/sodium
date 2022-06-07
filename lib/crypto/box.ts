import bindings from '../bindings';
import type { KeyPair, CryptoBox } from '../types';

const box = new bindings.Box();

export const crypto_box_beforenm = (pk: Uint8Array, sk: Uint8Array): Uint8Array => box.crypto_box_beforenm(pk, sk);
export const crypto_box_detached = (m: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): CryptoBox => box.crypto_box_detached(m, n, pk, sk);
export const crypto_box_easy = (m: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array => box.crypto_box_easy(m, n, pk, sk);
export const crypto_box_keypair = (): KeyPair => box.crypto_box_keypair();
export const crypto_box_nonce = (): Uint8Array => box.crypto_box_nonce();
export const crypto_box_open_detached = (c: Uint8Array, mac: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array =>
	box.crypto_box_open_detached(c, mac, n, pk, sk);

export const crypto_box_open_detached_afternm = (c: Uint8Array, mac: Uint8Array, n: Uint8Array, k: Uint8Array): Uint8Array =>
	box.crypto_box_open_detached_afternm(c, mac, n, k);

export const crypto_box_open_easy = (c: Uint8Array, n: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array =>
	box.crypto_box_open_easy(c, n, pk, sk);

export const crypto_box_seal = (m: Uint8Array, pk: Uint8Array): Uint8Array => box.crypto_box_seal(m, pk);
export const crypto_box_seal_open = (c: Uint8Array, pk: Uint8Array, sk: Uint8Array): Uint8Array => box.crypto_box_seal_open(c, pk, sk);
export const crypto_box_seed_keypair = (seed: Uint8Array): KeyPair => box.crypto_box_seed_keypair(seed);
