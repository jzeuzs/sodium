import { add, complete, cycle, suite } from 'benny';
import * as sodium from '../dist';
import libsodiumWrappers from 'libsodium-wrappers';
import tweetnacl from 'tweetnacl';
import chloride from 'chloride';

let nodeSodium: any;

try {
	nodeSodium = require('sodium');
} catch {}

if (nodeSodium) {
	void suite(
		'keypair generation',
		add('libsodium-wrappers', async () => {
			await libsodiumWrappers.ready;
			libsodiumWrappers.crypto_box_keypair();
		}),
		add('tweetnacl', () => tweetnacl.box.keyPair()),
		add('chloride', () => chloride.crypto_box_keypair()),
		add('sodium', () => nodeSodium.api.crypto_box_keypair()),
		add('@devtomio/sodium', () => sodium.crypto_box_keypair()),
		cycle(),
		complete()
	);
} else {
	void suite(
		'keypair generation',
		add('libsodium-wrappers', async () => {
			await libsodiumWrappers.ready;
			libsodiumWrappers.crypto_box_keypair();
		}),
		add('tweetnacl', () => tweetnacl.box.keyPair()),
		add('chloride', () => chloride.crypto_box_keypair()),
		add('@devtomio/sodium', () => sodium.crypto_box_keypair()),
		cycle(),
		complete()
	);
}
