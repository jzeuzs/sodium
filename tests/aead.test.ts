import * as sodium from '../dist';

test('crypto_aead_xchacha20poly1305_ietf_*', () => {
	const plaintext = Buffer.from(
		'4c616469657320616e642047656e746c656d656e206f662074686520636c6173' +
			'73206f66202739393a204966204920636f756c64206f6666657220796f75206f' +
			'6e6c79206f6e652074697020666f7220746865206675747572652c2073756e73' +
			'637265656e20776f756c642062652069742e',
		'hex'
	);

	const assocData = Buffer.from('50515253c0c1c2c3c4c5c6c7', 'hex');
	const nonce = Buffer.from('404142434445464748494a4b4c4d4e4f5051525354555657', 'hex');
	const key = Buffer.from('808182838485868788898a8b8c8d8e8f909192939495969798999a9b9c9d9e9f', 'hex');
	const ciphertext = Buffer.from(sodium.crypto_aead_xchacha20poly1305_ietf_encrypt(plaintext, assocData, null, nonce, key));
	const expected =
		'bd6d179d3e83d43b9576579493c0e939572a1700252bfaccbed2902c21396cbb' +
		'731c7f1b0b4aa6440bf3a82f4eda7e39ae64c6708c54c216cb96b72e1213b452' +
		'2f8c9ba40db5d945b11b69b982c1bb9e3f3fac2bc369488f76b2383565d3fff9' +
		'21f9664c97637da9768812f615c68b13b52e' +
		'c0875924c1c7987947deafd8780acf49';

	expect(ciphertext.toString('hex')).toBe(expected);

	let decrypted = Buffer.from(sodium.crypto_aead_xchacha20poly1305_ietf_decrypt(null, ciphertext, assocData, nonce, key));
	expect(decrypted.toString('hex')).toBe(plaintext.toString('hex'));

	const randomKey = Buffer.from(sodium.crypto_aead_xchacha20poly1305_ietf_keygen());
	expect(randomKey).toBeInstanceOf(Buffer);

	const ciphertext2 = Buffer.from(sodium.crypto_aead_xchacha20poly1305_ietf_encrypt(plaintext, null, null, nonce, randomKey));
	decrypted = Buffer.from(sodium.crypto_aead_xchacha20poly1305_ietf_decrypt(null, ciphertext2, null, nonce, randomKey));

	expect(decrypted.toString('hex')).toBe(plaintext.toString('hex'));
	expect(ciphertext.toString('hex')).not.toBe(ciphertext2.toString('hex'));
});
