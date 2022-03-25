import conventionalChangelog from 'conventional-changelog';

conventionalChangelog({ preset: 'angular' }).pipe(process.stdout);
