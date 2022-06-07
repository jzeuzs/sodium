import bindings from '../bindings';

export const randombytes_buf = (len: number): Uint8Array => bindings.randombytes_buf(len);
