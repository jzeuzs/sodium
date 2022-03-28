import * as sodium from '../dist';

test('sign', () => {
	const aliceKeypair = sodium.crypto_sign_keypair();
	const aliceSecret = aliceKeypair.secret_key;
	const alicePublic = aliceKeypair.public_key;

	const plaintext = 'Science, math, technology, engineering, and compassion for others.';
	const signed = sodium.crypto_sign(Buffer.from(plaintext), aliceSecret);
	const opened = sodium.crypto_sign_open(signed, alicePublic);

	expect(signed.slice(64).toString()).toBe(opened.toString());
	expect(Buffer.from(opened).toString()).toBe(plaintext);

	const signature = sodium.crypto_sign_detached(Buffer.from(plaintext), aliceSecret);
	const valid = sodium.crypto_sign_verify_detached(signature, Buffer.from(plaintext), alicePublic);

	expect(valid).toBe(true);

	const invalid = sodium.crypto_sign_verify_detached(signature, Buffer.from(`${plaintext} extra`), alicePublic);

	expect(invalid).toBe(false);
});
