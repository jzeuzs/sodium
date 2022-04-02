import * as sodium from '../dist';

test('generichash', () => {
	const message = 'Science, math, technology, engineering, and compassion for others.';
	const piece1 = message.slice(0, 16);
	const piece2 = message.slice(16);

	let hash1 = Buffer.from(sodium.crypto_generichash(Buffer.from(message), 32, null));
	expect(hash1.toString('hex')).toBe('47c1fdbde32b30b9c54dd47cf88ba92d2d05df1265e342c9563ed56aee84ab02');

	let state = sodium.crypto_generichash_init(32, null);
	let state1 = sodium.crypto_generichash_update(state, Buffer.from(piece1));
	let state2 = sodium.crypto_generichash_update(state1, Buffer.from(piece2));
	let hash2 = Buffer.from(sodium.crypto_generichash_final(state2, 32));
	expect(hash2.toString('hex', 0, 32)).toBe(hash1.toString('hex', 0, 32));

	const key = Buffer.from(sodium.crypto_generichash_keygen());
	hash1 = Buffer.from(sodium.crypto_generichash(Buffer.from(message), 32, key));
	state = sodium.crypto_generichash_init(32, key);
	state1 = sodium.crypto_generichash_update(state, Buffer.from(piece1));
	state2 = sodium.crypto_generichash_update(state1, Buffer.from(piece2));
	hash2 = Buffer.from(sodium.crypto_generichash_final(state2, 32));

	expect(hash1.toString('hex', 0, 32)).toBe(hash2.toString('hex', 0, 32));
});
