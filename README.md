<div align="center">

# @devtomio/sodium

**<a href="https://libsodium.gitbook.io">Libsodium</a> for <a href="https://nodejs.org">Node.js</a>**

[![GitHub](https://img.shields.io/github/license/devtomio/sodium)](https://github.com/devtomio/sodium/blob/main/LICENSE.md)
[![npm](https://img.shields.io/npm/v/@devtomio/sodium?color=crimson&logo=npm&style=flat-square)](https://www.npmjs.com/package/@devtomio/sodium)

</div>

## Features

- Fully ready for TypeScript!
- Includes ESM ready entrypoint
- Easy to use
- Has the same API as libsodium
- Faster than similar libraries (see [benchmarks](#benchmarks))

---

## Usage

**_For complete usages, please dive into our [documentation]_**

### Basic usage

Generating a keypair

```typescript
import * as sodium from '@devtomio/sodium';

const { public_key, secret_key } = sodium.crypto_box_keypair();
```

## Benchmarks

```sh
$ yarn bench

Running "keypair generation" suite...
Progress: 100%

  libsodium-wrappers:
    3 532 ops/s, ±7.21%    | 87.99% slower

  tweetnacl:
    859 ops/s, ±3.72%      | slowest, 97.08% slower

  @devtomio/sodium:
    29 402 ops/s, ±5.38%   | fastest

Finished 3 cases!
  Fastest: @devtomio/sodium
  Slowest: tweetnacl
```

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tr>
    <td align="center"><a href="https://tomio.codes/"><img src="https://avatars.githubusercontent.com/u/75403863?v=4?s=100" width="100px;" alt=""/><br /><sub><b>Tomio</b></sub></a><br /><a href="https://github.com/devtomio/sodium/commits?author=devtomio" title="Code">💻</a> <a href="https://github.com/devtomio/sodium/commits?author=devtomio" title="Documentation">📖</a> <a href="#example-devtomio" title="Examples">💡</a> <a href="#ideas-devtomio" title="Ideas, Planning, & Feedback">🤔</a> <a href="#infra-devtomio" title="Infrastructure (Hosting, Build-Tools, etc)">🚇</a> <a href="#maintenance-devtomio" title="Maintenance">🚧</a> <a href="#platform-devtomio" title="Packaging/porting to new platform">📦</a></td>
    <td align="center"><a href="https://renovate.whitesourcesoftware.com/"><img src="https://avatars.githubusercontent.com/u/25180681?v=4?s=100" width="100px;" alt=""/><br /><sub><b>WhiteSource Renovate</b></sub></a><br /><a href="#maintenance-renovate-bot" title="Maintenance">🚧</a></td>
  </tr>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind welcome!

[documentation]: https://devtomio.github.io/sodium
