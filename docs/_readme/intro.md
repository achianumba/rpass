<!-- markdownlint-disable no-inline-html -->
# rpass

![Crates.io Total Downloads](https://img.shields.io/crates/d/rpass)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rpass/latest)
![Crates.io Size](https://img.shields.io/crates/size/rpass)

`rpass` is a [`pass`](https://www.passwordstore.org/) inspired secrets manager.

> [!WARNING]
> ⚠️ THIS PROJECT IS AN ACTIVE WORK-IN-PROGRESS.
>
> ⚠️ CORE COMMANDS WORK BUT MAY BREAK BEAUSE THIS PROJECT IS STILL EVOLVING.
>
> ⚠️ SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES.

`rpass list` output:

<pre>
rPass Store
├── some-service
│   └── username@some-service.com
└── example
    └── bob@example.com
</pre>

`tree` output:

<pre>
.rstore
├── 3c11af1b-2c11-411a-bc4b-9e2aef34a928
│   └── 26ee01a5-180d-4aee-9d03-2d83154c989b.gpg
├── b206d775-8adc-4e1d-9f30-88a6cd6f1a10
│   └── 67da9fe5-81fa-4990-a022-25623b788128.gpg
└── store.toml
</pre>

## Features

- Asymmetric encryption
- Symmetric encryption
- Entry anonymization
- Clipboard support
- Git integration
- Random password generation
- Random passphrase generation
- Manage multiple stores