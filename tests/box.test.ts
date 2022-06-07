import * as sodium from '../dist';

test('box', () => {
	const plaintext = 'Science, math, technology, engineering, and compassion for others.';

	const aliceKeypair = sodium.crypto_box_keypair();
	const aliceSecret = aliceKeypair.secret_key;
	const alicePublic = aliceKeypair.public_key;
	const bobKeypair = sodium.crypto_box_keypair();
	const bobSecret = bobKeypair.secret_key;
	const bobPublic = bobKeypair.public_key;
	const nonce = sodium.crypto_box_nonce();

	const ciphertext = sodium.crypto_box_easy(Buffer.from(plaintext), nonce, bobPublic, aliceSecret);
	const decrypted = sodium.crypto_box_open_easy(ciphertext, nonce, alicePublic, bobSecret);

	expect(Buffer.from(decrypted).toString('hex')).toBe(Buffer.from(plaintext).toString('hex'));
});
