export type Nullish<T> = T | null | undefined;

export interface KeyPair {
	public_key: Uint8Array;
	secret_key: Uint8Array;
}

export interface CryptoBox {
	ciphertext: Uint8Array;
	mac: Uint8Array;
}
