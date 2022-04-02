import { loadBinding } from '@node-rs/helper';
import { join } from 'node:path';

const bindings: { [key: string]: any } = loadBinding(join(__dirname, '..'), 'sodium', '@devtomio/sodium');
const { Box, randombytes_buf, SecretBox, Sign, GenericHash } = bindings;

export { Box, randombytes_buf, SecretBox, Sign, GenericHash };
