<!-- markdownlint-disable no-inline-html -->

# rpass

[![Latest Version](https://img.shields.io/crates/v/rpass.svg)](https://crates.io/crates/rpass)
![Crates.io Total Downloads](https://img.shields.io/crates/d/rpass)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/rpass/latest)
![Crates.io Size](https://img.shields.io/crates/size/rpass)

`rpass` is a GPG-based secrets manager with the following features:

<details>
<summary>Cross-platform support</summary>

`rpass` currently works on x86_64 Linux and Windows but hasn't been tested on other platforms.
</details>

<details>
<summary>Asymmetric/Symmetric encryption</summary>
You have the option of using a store of symmetricly or asymmetrically encrypted 
secrets depending on whether the store is initialized using the `-k` option.

Inputs and outputs for encryption and decryption operations are passed through pipes to and from a call to `gpg` call in a child process. 
</details>

<details>
<summary>UUID-based entry anonymization</summary>
<table>
<tr>
<th><code>rpass</code> list</th>
<th><code>tree</code> output</th>
</tr>

<tr>
<td>
<pre>
rPass Store
├── some-service
│   └── username@some-service.com
└── example
    └── bob@example.com
</pre>
</td>

<td>
<pre>
.rstore
├── 3c11af1b-2c11-411a-bc4b-9e2aef34a928
│   └── 26ee01a5-180d-4aee-9d03-2d83154c989b.gpg
├── b206d775-8adc-4e1d-9f30-88a6cd6f1a10
│   └── 67da9fe5-81fa-4990-a022-25623b788128.gpg
└── store.toml
</pre>
</td>
</tr>
</table>
</details>

<details>
<summary>Random password/passphrase generation</summary>

Random passwords are generated from [printable ASCII characters](https://en.wikipedia.org/wiki/ASCII#Printable_character_table) while random passphrases are generated from [EFF's large word list](https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt).

</details>

<details>
<summary>Multi-store management</summary>

Maintain multiple stores in different locations or change the store's default location by passing the global `STORE` CLI option
or by setting the `DEFAULT_RPASS_STORE` environment variable.

</details>

<details>
<summary>Git integration</summary>

`rpass` commits generic info for each change to an entry or to the store in general without revealing specific information about entries.
</details>

<details>
<summary>Clipboard support</summary>

Subcommands such as `show` optionally copy secrets to the clipboard.
</details>
<br>

> [!WARNING]
> ⚠️ THIS PROJECT IS AN ACTIVE WORK-IN-PROGRESS.
>
> ⚠️ CORE COMMANDS WORK BUT MAY BREAK BEAUSE THIS PROJECT IS STILL EVOLVING.
>
> ⚠️ SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES.
