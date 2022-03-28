import { add, complete, cycle, suite } from 'benny';
import * as sodium from '../dist';
import libsodiumWrappers from 'libsodium-wrappers';
import tweetnacl from 'tweetnacl';
import chloride from 'chloride';

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
