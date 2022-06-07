import bindings from '../bindings';
import type { Nullish, CryptoBox } from '../types';

const aead = new bindings.Aead();

export const crypto_aead_chacha20poly1305_decrypt = (
	sn: Nullish<Uint8Array>,
	c: Uint8Array,
	ad: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_chacha20poly1305_decrypt(sn, c, ad, pn, key);

export const crypto_aead_chacha20poly1305_decrypt_detached = (
	sn: Nullish<Uint8Array>,
	c: Uint8Array,
	mac: Uint8Array,
	ad: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_chacha20poly1305_decrypt_detached(sn, c, mac, ad, pn, key);

export const crypto_aead_chacha20poly1305_encrypt = (
	m: Uint8Array,
	ad: Nullish<Uint8Array>,
	sn: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_chacha20poly1305_encrypt(m, ad, sn, pn, key);

export const crypto_aead_chacha20poly1305_encrypt_detached = (
	m: Uint8Array,
	ad: Nullish<Uint8Array>,
	sn: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): CryptoBox => aead.crypto_aead_chacha20poly1305_encrypt_detached(m, ad, sn, pn, key);

export const crypto_aead_chacha20poly1305_ietf_decrypt = (
	sn: Nullish<Uint8Array>,
	c: Uint8Array,
	ad: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_chacha20poly1305_ietf_decrypt(sn, c, ad, pn, key);

export const crypto_aead_chacha20poly1305_ietf_decrypt_detached = (
	sn: Nullish<Uint8Array>,
	c: Uint8Array,
	mac: Uint8Array,
	ad: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_chacha20poly1305_ietf_decrypt_detached(sn, c, mac, ad, pn, key);

export const crypto_aead_chacha20poly1305_ietf_encrypt = (
	m: Uint8Array,
	ad: Nullish<Uint8Array>,
	sn: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_chacha20poly1305_ietf_encrypt(m, ad, sn, pn, key);

export const crypto_aead_chacha20poly1305_ietf_encrypt_detached = (
	m: Uint8Array,
	ad: Nullish<Uint8Array>,
	sn: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): CryptoBox => aead.crypto_aead_chacha20poly1305_ietf_encrypt_detached(m, ad, sn, pn, key);

export const crypto_aead_chacha20poly1305_ietf_keygen = (): Uint8Array => aead.crypto_aead_chacha20poly1305_ietf_keygen();
export const crypto_aead_chacha20poly1305_keygen = (): Uint8Array => aead.crypto_aead_chacha20poly1305_keygen();
export const crypto_aead_xchacha20poly1305_ietf_decrypt = (
	sn: Nullish<Uint8Array>,
	c: Uint8Array,
	ad: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_xchacha20poly1305_ietf_decrypt(sn, c, ad, pn, key);

export const crypto_aead_xchacha20poly1305_ietf_decrypt_detached = (
	sn: Nullish<Uint8Array>,
	c: Uint8Array,
	mac: Uint8Array,
	ad: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_xchacha20poly1305_ietf_decrypt_detached(sn, c, mac, ad, pn, key);

export const crypto_aead_xchacha20poly1305_ietf_encrypt = (
	m: Uint8Array,
	ad: Nullish<Uint8Array>,
	sn: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): Uint8Array => aead.crypto_aead_xchacha20poly1305_ietf_encrypt(m, ad, sn, pn, key);

export const crypto_aead_xchacha20poly1305_ietf_encrypt_detached = (
	m: Uint8Array,
	ad: Nullish<Uint8Array>,
	sn: Nullish<Uint8Array>,
	pn: Uint8Array,
	key: Uint8Array
): CryptoBox => aead.crypto_aead_xchacha20poly1305_ietf_encrypt_detached(m, ad, sn, pn, key);

export const crypto_aead_xchacha20poly1305_ietf_keygen = (): Uint8Array => aead.crypto_aead_xchacha20poly1305_ietf_keygen();
