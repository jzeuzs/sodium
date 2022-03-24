import { promisify } from 'node:util';
import semverRegex from 'semver-regex';
import publish from '@jsdevtools/npm-publish';
import { fileURLToPath } from 'node:url';
import { exec } from 'node:child_process';

const e = promisify(exec);
const commit = await e('git log -1 --pretty=%B', { encoding: 'utf-8' });
const isSemver = semverRegex().test(commit.stdout);

if (isSemver) {
	await publish({ token: process.env.NPM_TOKEN, package: fileURLToPath(new URL('../package.json', import.meta.url)) });
	console.log('Published!');
} else console.log('Not a release');
