import { GenericHash } from '../bindings';
import type { Nullish } from '../types';

const generichash = new GenericHash();

export interface GenericHashState {
	opaque: Uint8Array;
}

export const crypto_generichash_BYTES = generichash.cryptoGenerichashBytes;
export const crypto_generichash_BYTES_MAX = generichash.cryptoGenerichashBytesMax;
export const crypto_generichash_BYTES_MIN = generichash.cryptoGenerichashBytesMin;
export const crypto_generichash_KEYBYTES = generichash.cryptoGenerichashKeybytes;
export const crypto_generichash_KEYBYTES_MAX = generichash.cryptoGenerichashKeybytesMax;
export const crypto_generichash_KEYBYTES_MIN = generichash.cryptoGenerichashKeybytesMin;

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/generichash/fn.hash.html */
export const crypto_generichash = (data: Uint8Array, out_len: Nullish<number>, key: Nullish<Uint8Array>): Uint8Array =>
	generichash.crypto_generichash(data, out_len, key);

/** @see https://docs.rs/libsodium-sys/latest/libsodium_sys/fn.crypto_generichash_final.html */
export const crypto_generichash_final = (state: GenericHashState, out_len: number): Uint8Array =>
	generichash.crypto_generichash_final(state, out_len);

/** @see https://docs.rs/sodiumoxide/latest/sodiumoxide/crypto/generichash/struct.State.html#method.new */
export const crypto_generichash_init = (out_len: Nullish<number>, key: Nullish<Uint8Array>): GenericHashState =>
	generichash.crypto_generichash_init(out_len, key);

/** @see https://docs.rs/libsodium-sys/latest/libsodium_sys/fn.crypto_generichash_keygen.html */
export const crypto_generichash_keygen = (): Uint8Array => generichash.crypto_generichash_keygen();

/** @see https://docs.rs/libsodium-sys/latest/libsodium_sys/fn.crypto_generichash_update.html */
export const crypto_generichash_update = (state: GenericHashState, data: Uint8Array): GenericHashState =>
	generichash.crypto_generichash_update(state, data);
