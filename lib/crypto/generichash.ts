import bindings from '../bindings';
import type { Nullish } from '../types';

const generichash = new bindings.GenericHash();

export interface GenericHashState {
	opaque: Uint8Array;
}

export const crypto_generichash = (data: Uint8Array, out_len: Nullish<number>, key: Nullish<Uint8Array>): Uint8Array =>
	generichash.crypto_generichash(data, out_len, key);

export const crypto_generichash_final = (state: GenericHashState, out_len: number): Uint8Array =>
	generichash.crypto_generichash_final(state, out_len);

export const crypto_generichash_init = (out_len: Nullish<number>, key: Nullish<Uint8Array>): GenericHashState =>
	generichash.crypto_generichash_init(out_len, key);

export const crypto_generichash_keygen = (): Uint8Array => generichash.crypto_generichash_keygen();
export const crypto_generichash_update = (state: GenericHashState, data: Uint8Array): GenericHashState =>
	generichash.crypto_generichash_update(state, data);
