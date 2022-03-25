import conventionalChangelog from 'conventional-changelog';
import { stdout } from 'node:process';

conventionalChangelog().pipe(stdout);
