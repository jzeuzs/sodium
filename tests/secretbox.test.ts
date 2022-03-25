import * as sodium from '../dist';

test('secretbox', () => {
	const plaintext = 'Science, math, technology, engineering, and compassion for others.';
	const key = sodium.crypto_secretbox_keygen();
	const nonce = sodium.crypto_secretbox_nonce();
	const ciphertext = sodium.crypto_secretbox_easy(Buffer.from(plaintext), nonce, key);
	const decrypted = sodium.crypto_secretbox_open_easy(ciphertext, nonce, key);

	expect(Buffer.from(decrypted).toString('hex')).toBe(Buffer.from(plaintext).toString('hex'));
});
