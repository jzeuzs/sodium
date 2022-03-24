import git from 'git-last-commit';
import { promisify } from 'node:util';
import semverRegex from 'semver-regex';
import publish from '@jsdevtools/npm-publish';
import { fileURLToPath } from 'node:url';

const getLastCommit = promisify(git.getLastCommit);
const { body } = await getLastCommit();
const isSemver = semverRegex().test(body);

if (isSemver) await publish({ token: process.env.NPM_TOKEN, package: fileURLToPath(new URL('../package.json', import.meta.url)) });
else console.log('No');
