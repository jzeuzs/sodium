import { existsSync, readFileSync } from 'node:fs';
import { join } from 'node:path';

const loadBinding = () => {
	const { platform, arch } = process;
	let nativeBinding = null;
	let localFileExisted = false;
	let isMusl = false;
	let loadError = null;

	switch (platform) {
		case 'android': {
			if (arch !== 'arm64') throw new Error(`Unsupported architecture on Android: ${arch}`);
			localFileExisted = existsSync(join(__dirname, '..', 'sodium.android-arm64.node'));

			try {
				if (localFileExisted) nativeBinding = require('../sodium.android-arm64.node');
				else nativeBinding = require('@devtomio/sodium-android-arm64');
			} catch (e) {
				loadError = e;
			}

			break;
		}
		case 'win32': {
			switch (arch) {
				case 'x64': {
					localFileExisted = existsSync(join(__dirname, '..', 'sodium.win32-x64-msvc.node'));

					try {
						if (localFileExisted) nativeBinding = require('../sodium.win32-x64-msvc.node');
						else nativeBinding = require('@devtomio/sodium-win32-x64-msvc');
					} catch (e) {
						loadError = e;
					}

					break;
				}
				case 'ia32': {
					localFileExisted = existsSync(join(__dirname, '..', 'sodium.win32-ia32-msvc.node'));

					try {
						if (localFileExisted) nativeBinding = require('../sodium.win32-ia32-msvc.node');
						else nativeBinding = require('@devtomio/sodium-win32-ia32-msvc');
					} catch (e) {
						loadError = e;
					}

					break;
				}
				case 'arm64': {
					localFileExisted = existsSync(join(__dirname, '..', 'sodium.win32-arm64-msvc.node'));

					try {
						if (localFileExisted) nativeBinding = require('../sodium.win32-arm64-msvc.node');
						else nativeBinding = require('@devtomio/sodium-win32-arm64-msvc');
					} catch (e) {
						loadError = e;
					}

					break;
				}
				default:
					throw new Error(`Unsupported architecture in Windows: ${arch}`);
			}

			break;
		}
		case 'darwin': {
			switch (arch) {
				case 'x64': {
					localFileExisted = existsSync(join(__dirname, '..', 'sodium.darwin-x64.node'));

					try {
						if (localFileExisted) nativeBinding = require('../sodium.darwin-x64.node');
						else nativeBinding = require('@devtomio/sodium-darwin-x64');
					} catch (e) {
						loadError = e;
					}

					break;
				}
				case 'arm64': {
					localFileExisted = existsSync(join(__dirname, '..', 'sodium.darwin-arm64.node'));

					try {
						if (localFileExisted) nativeBinding = require('../sodium.darwin-arm64.node');
						else nativeBinding = require('@devtomio/sodium-darwin-arm64');
					} catch (e) {
						loadError = e;
					}

					break;
				}
				default:
					throw new Error(`Unsupported architecture on MacOS: ${arch}`);
			}

			break;
		}
		case 'freebsd': {
			if (arch !== 'x64') throw new Error(`Unsupported architecture on FreeBSD: ${arch}`);

			localFileExisted = existsSync(join(__dirname, '..', 'sodium.freebsd-x64.node'));

			try {
				if (localFileExisted) nativeBinding = require('../sodium.freebsd-x64.node');
				else nativeBinding = require('@devtomio/sodium-freebsd-x64');
			} catch (e) {
				loadError = e;
			}

			break;
		}
		case 'linux': {
			switch (arch) {
				case 'x64': {
					isMusl = readFileSync('/usr/bin/ldd', 'utf8').includes('musl');

					if (isMusl) {
						localFileExisted = existsSync(join(__dirname, '..', 'sodium.linux-x64-musl.node'));

						try {
							if (localFileExisted) nativeBinding = require('../sodium.linux-x64-musl.node');
							else nativeBinding = require('@devtomio/sodium-linux-x64-musl');
						} catch (e) {
							loadError = e;
						}
					} else {
						localFileExisted = existsSync(join(__dirname, '..', 'sodium.linux-x64-gnu.node'));

						try {
							if (localFileExisted) nativeBinding = require('../sodium.linux-x64-gnu.node');
							else nativeBinding = require('@devtomio/sodium-linux-x64-gnu');
						} catch (e) {
							loadError = e;
						}
					}

					break;
				}
				case 'arm64': {
					isMusl = readFileSync('/usr/bin/ldd', 'utf8').includes('musl');

					if (isMusl) {
						localFileExisted = existsSync(join(__dirname, '..', 'sodium.linux-arm64-musl.node'));

						try {
							if (localFileExisted) nativeBinding = require('../sodium.linux-arm64-musl.node');
							else nativeBinding = require('@devtomio/sodium-linux-arm64-musl');
						} catch (e) {
							loadError = e;
						}
					} else {
						localFileExisted = existsSync(join(__dirname, '..', 'sodium.linux-arm64-gnu.node'));

						try {
							if (localFileExisted) nativeBinding = require('../sodium.linux-arm64-gnu.node');
							else nativeBinding = require('@devtomio/sodium-linux-arm64-gnu');
						} catch (e) {
							loadError = e;
						}
					}

					break;
				}
				case 'arm': {
					localFileExisted = existsSync(join(__dirname, '..', 'sodium.linux-arm-gnueabihf.node'));

					try {
						if (localFileExisted) nativeBinding = require('../sodium.linux-arm-gnueabihf.node');
						else nativeBinding = require('@devtomio/sodium-linux-arm-gnueabihf');
					} catch (e) {
						loadError = e;
					}

					break;
				}
				default:
					throw new Error(`Unsupported architecture on Linux: ${arch}`);
			}

			break;
		}
		default:
			throw new Error(`Unsupported OS: ${platform}, arch: ${arch}`);
	}

	localFileExisted = existsSync(join(__dirname, '..', 'sodium.node'));

	try {
		if (localFileExisted) nativeBinding = require('../sodium.node');
	} catch (e) {
		loadError = e;
	}

	if (!nativeBinding) {
		if (loadError) throw loadError;

		throw new Error('Failed to load native binding.');
	}

	return nativeBinding;
};

const bindings: { [key: string]: any } = loadBinding();
const { Box } = bindings;

export { Box };
