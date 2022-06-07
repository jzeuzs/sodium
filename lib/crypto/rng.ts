import bindings from '../bindings';

/**
 * Provides random data up to `len` from the OS’s random number generator.
 * @see {@link https://docs.rs/dryoc/0.3.12/dryoc/rng/fn.randombytes_buf.html rng::randombytes_buf}
 */
export const randombytes_buf = (len: number): Uint8Array => bindings.randombytes_buf(len);
