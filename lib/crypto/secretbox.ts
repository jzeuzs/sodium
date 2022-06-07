import bindings from '../bindings';
import type { CryptoBox as SecretBox } from '../types';

const secretbox = new bindings.SecretBox();

export const crypto_secretbox_detached = (m: Uint8Array, n: Uint8Array, k: Uint8Array): SecretBox => secretbox.crypto_secretbox_detached(m, n, k);
export const crypto_secretbox_easy = (m: Uint8Array, n: Uint8Array, k: Uint8Array): Uint8Array => secretbox.crypto_secretbox_easy(m, n, k);
export const crypto_secretbox_keygen = (): Uint8Array => secretbox.crypto_secretbox_keygen();
export const crypto_secretbox_nonce = (): Uint8Array => secretbox.crypto_secretbox_nonce();
export const crypto_secretbox_open_detached = (c: Uint8Array, mac: Uint8Array, n: Uint8Array, k: Uint8Array): Uint8Array =>
	secretbox.crypto_secretbox_open_detached(c, mac, n, k);

export const crypto_secretbox_open_easy = (c: Uint8Array, n: Uint8Array, k: Uint8Array): Uint8Array => secretbox.crypto_secretbox_open_easy(c, n, k);
