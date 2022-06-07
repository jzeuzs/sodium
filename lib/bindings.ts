import { loadBinding } from '@node-rs/helper';
import { join } from 'node:path';

const bindings: { [key: string]: any } = loadBinding(join(__dirname, '..'), 'sodium', '@devtomio/sodium');

export default bindings;
