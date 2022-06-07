import bindings from '../bindings';
import type { KeyPair } from '../types';

const sign = new bindings.Sign();

export interface SignHash {
	state: BigUint64Array;
	count: BigUint64Array;
	buf: Uint8Array;
}

export interface SignState {
	hs: SignHash;
}

export const crypto_sign = (m: Uint8Array, sk: Uint8Array): Uint8Array => sign.crypto_sign(m, sk);
export const crypto_sign_detached = (m: Uint8Array, sk: Uint8Array): Uint8Array => sign.crypto_sign_detached(m, sk);
export const crypto_sign_ed25519_pk_to_curve25519 = (ed25519_pk: Uint8Array): Uint8Array => sign.crypto_sign_ed25519_pk_to_curve25519(ed25519_pk);
export const crypto_sign_ed25519_sk_to_curve25519 = (ed25519_sk: Uint8Array): Uint8Array => sign.crypto_sign_ed25519_sk_to_curve25519(ed25519_sk);
export const crypto_sign_final_create = (state: SignState, sk: Uint8Array): Uint8Array => sign.crypto_sign_final_create(state, sk);
export const crypto_sign_final_verify = (state: SignState, sig: Uint8Array, pk: Uint8Array): boolean => sign.crypto_sign_final_verify(state, sig, pk);
export const crypto_sign_init = (): SignState => sign.crypto_sign_init();
export const crypto_sign_keypair = (): KeyPair => sign.crypto_sign_keypair();
export const crypto_sign_open = (sm: Uint8Array, pk: Uint8Array): Uint8Array => sign.crypto_sign_open(sm, pk);
export const crypto_sign_seed_keypair = (seed: Uint8Array): KeyPair => sign.crypto_sign_seed_keypair(seed);
export const crypto_sign_update = (state: SignState, m: Uint8Array): SignState => sign.crypto_sign_update(state, m);
export const crypto_sign_verify_detached = (sig: Uint8Array, m: Uint8Array, pk: Uint8Array): boolean => sign.crypto_sign_verify_detached(sig, m, pk);
