import conventionalChangelog from 'conventional-changelog';

conventionalChangelog({ preset: 'angular', outputUnreleased: true }).pipe(process.stdout);
